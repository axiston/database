-- Manages user account information.
CREATE TABLE accounts
(
    -- Unique identifier for each account, used as a public resource.
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- User-provided account name, typically the full name.
    display_name  TEXT      NOT NULL,

    -- User-provided unique email address, ignoring deactivated accounts.
    email_address TEXT      NOT NULL,
    -- Hashed version of the user-provided password (or random for OAuth).
    password_hash TEXT      NOT NULL,

    -- TODO: Implement email validation (e.g., is_validated or validate_at).

    -- Prevents empty values for critical fields.
    CONSTRAINT accounts_non_empty_display_name CHECK (display_name <> ''),
    CONSTRAINT accounts_non_empty_email_address CHECK (email_address <> ''),
    CONSTRAINT accounts_non_empty_password_hash CHECK (password_hash <> ''),

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT accounts_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT accounts_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT accounts_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` field when a row is modified.
SELECT manage_updated_at('accounts');

-- Ensures email addresses are unique among active accounts.
CREATE UNIQUE INDEX accounts_email_address_idx
    ON accounts (email_address)
    WHERE deleted_at IS NULL;

-- Optimizes lookup for active accounts using email and password.
CREATE INDEX accounts_credentials_idx
    ON accounts (email_address, password_hash)
    WHERE deleted_at IS NULL;

-- Manages active user sessions and their metadata.
CREATE TABLE account_sessions
(
    -- Unique identifier for each session token.
    token_seq  UUID      NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Two-character region code (e.g., "US", "EU").
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

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_sessions_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_sessions_deleted_after_issued CHECK (deleted_at IS NULL OR deleted_at >= issued_at)
);

-- Optimizes lookup for active sessions using account ID and token.
CREATE INDEX account_sessions_active_idx
    ON account_sessions (account_id, token_seq)
    WHERE deleted_at IS NULL;

-- Manages user permissions for read and write operations.
CREATE TABLE account_permissions
(
    -- Reference to the associated account.
    account_id    UUID PRIMARY KEY REFERENCES accounts (id) ON DELETE CASCADE,

    -- Universal permissions flags for read and write access.
    nocheck_read  BOOLEAN   NOT NULL DEFAULT FALSE,
    nocheck_write BOOLEAN   NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_permissions_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT account_permissions_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT account_permissions_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` field when a row is modified.
SELECT manage_updated_at('account_permissions');

-- Optimizes lookup for active permissions using account ID.
CREATE INDEX account_permissions_idx
    ON account_permissions (account_id)
    WHERE deleted_at IS NULL;

-- Defines possible actions confirmed via action tokens.
CREATE TYPE TOKEN_ACTION AS ENUM ('confirm_email', 'update_email', 'reset_password');

-- Manages issued tokens for confirmed actions.
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

    -- Restricts the size of the token metadata.
    CONSTRAINT account_tokens_token_data_limit CHECK (length(token_data::TEXT) <= 2048),

    -- Security-related information.
    ip_address   INET         NOT NULL,
    user_agent   TEXT         NOT NULL,

    -- Timestamps for tracking the row's lifecycle.
    issued_at    TIMESTAMP    NOT NULL DEFAULT current_timestamp,
    expired_at   TIMESTAMP    NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    used_at      TIMESTAMP             DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_tokens_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_tokens_used_after_issued CHECK (used_at IS NULL OR used_at >= issued_at)
);

-- Optimizes lookup for unused tokens using account ID and token.
CREATE INDEX account_tokens_idx
    ON account_tokens (account_id, action_token)
    WHERE used_at IS NULL;
