-- Account data management with security and lifecycle tracking.
CREATE TABLE accounts
(
    -- Unique identifier for each account, used as a public resource.
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- Publicly visible name for user identification.
    display_name  TEXT      NOT NULL,
    -- Unique contact email, used for authentication and communication.
    email_address TEXT      NOT NULL,
    -- Securely hashed password to protect user credentials.
    password_hash TEXT      NOT NULL,

    --- Validation constraints to prevent empty or invalid entries.
    CONSTRAINT accounts_non_empty_display_name CHECK (display_name <> ''),
    CONSTRAINT accounts_non_empty_email_address CHECK (email_address <> ''),
    CONSTRAINT accounts_non_empty_password_hash CHECK (password_hash <> ''),

    -- Account activation status for additional security.
    is_activated  BOOL      NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMP          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT accounts_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT accounts_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT accounts_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('accounts');

-- Enforces unique email addresses for active accounts.
CREATE UNIQUE INDEX accounts_email_address_idx
    ON accounts (email_address)
    WHERE deleted_at IS NULL;

-- Optimizes authentication lookup performance.
CREATE INDEX accounts_credentials_idx
    ON accounts (email_address, password_hash)
    WHERE deleted_at IS NULL;

-- User session tracking with unique ID and security metadata.
CREATE TABLE account_sessions
(
    -- Unique session identifier with account association.
    token_seq  UUID      NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated user account.
    account_id UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Two-character region code for session origin (e.g., "US", "EU").
    region_id  CHAR(2)   NOT NULL DEFAULT 'A0',

    -- Ensures unique token sequences for each account.
    CONSTRAINT account_sessions_pkey PRIMARY KEY (account_id, token_seq),
    -- Region code must be alphanumeric and exactly two characters.
    CONSTRAINT account_sessions_region_alphanumeric CHECK (region_id ~ '^[A-Z0-9]{2}$'),

    -- Security-related information.
    ip_address INET      NOT NULL,
    user_agent TEXT      NOT NULL,

    -- Timestamps for tracking the row's lifecycle.
    issued_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    expired_at TIMESTAMP NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    deleted_at TIMESTAMP          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT account_sessions_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_sessions_deleted_after_issued CHECK (deleted_at IS NULL OR deleted_at >= issued_at)
);

-- Optimizes lookup for active sessions using session data.
CREATE INDEX account_sessions_active_idx
    ON account_sessions (account_id, token_seq)
    WHERE deleted_at IS NULL;

-- Granular user permissions for management.
CREATE TABLE account_permissions
(
    -- Reference to the associated account.
    account_id       UUID PRIMARY KEY REFERENCES accounts (id) ON DELETE CASCADE,

    -- Universal permissions flags for read and write access.
    read_accounts    BOOLEAN   NOT NULL DEFAULT FALSE,
    write_accounts   BOOLEAN   NOT NULL DEFAULT FALSE,

    read_workspaces  BOOLEAN   NOT NULL DEFAULT FALSE,
    write_workspaces BOOLEAN   NOT NULL DEFAULT FALSE,

    read_workflows   BOOLEAN   NOT NULL DEFAULT FALSE,
    write_workflows  BOOLEAN   NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking the row's lifecycle.
    created_at       TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at       TIMESTAMP NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT account_permissions_updated_after_created CHECK (updated_at >= created_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('account_permissions');

-- Defines. action token types for account operations.
CREATE TYPE TOKEN_ACTION AS ENUM (
    'activate_account', -- Verify and enable new account.
    'deactivate_account', -- Disable or suspend account.
    'update_email', -- Change registered email address.
    'reset_password' -- Initiate password recovery.
    );

-- Secure, time-limited action tokens for critical operations.
CREATE TABLE account_tokens
(
    -- Unique identifier for each action token.
    action_token UUID         NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Ensures unique token sequences for each account.
    CONSTRAINT account_tokens_pkey PRIMARY KEY (account_id, action_token),

    -- Specifies the type of action (e.g., confirm email, reset password).
    action_type  TOKEN_ACTION NOT NULL,
    -- Additional metadata for the token (e.g., source, target).
    token_data   JSONB        NOT NULL DEFAULT '{}'::JSONB,

    -- Restricts the size of the metadata field to avoid excessively large JSON data.
    CONSTRAINT account_tokens_token_data_limit CHECK (length(token_data::TEXT) <= 2048),

    -- Security-related information.
    ip_address   INET         NOT NULL,
    user_agent   TEXT         NOT NULL,

    -- Timestamps for tracking the row's lifecycle.
    issued_at    TIMESTAMP    NOT NULL DEFAULT current_timestamp,
    expired_at   TIMESTAMP    NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    used_at      TIMESTAMP             DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT account_tokens_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_tokens_used_after_issued CHECK (used_at IS NULL OR used_at >= issued_at),
    CONSTRAINT account_tokens_expired_after_used CHECK (expired_at IS NULL OR used_at IS NULL OR expired_at >= used_at)
);

-- Optimizes yet unused token retrieval using token data.
CREATE INDEX account_tokens_idx
    ON account_tokens (account_id, action_token)
    WHERE used_at IS NULL;
