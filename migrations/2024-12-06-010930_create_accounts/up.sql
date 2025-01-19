-- Manages user's account information.
CREATE TABLE accounts
(
    -- Unique identifier for each account (used as a public resource).
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- User-provided account name, typically a full name.
    display_name  TEXT      NOT NULL,
    -- User-provided unique email address (ignores deactivated accounts).
    email_address TEXT      NOT NULL,
    -- Hashed version of the user-provided (or random for oauth) password.
    password_hash TEXT      NOT NULL,

    -- Constraints to prevent empty fields.
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

-- Automatically updates the `updated_at` timestamp.
SELECT manage_updated_at('accounts');

-- Ensures unique email addresses for active accounts.
CREATE UNIQUE INDEX accounts_email_address_idx
    ON accounts (email_address)
    WHERE deleted_at IS NULL;

-- Optimizes lookup for active accounts by email and password.
CREATE INDEX accounts_credentials_idx
    ON accounts (email_address, password_hash)
    WHERE deleted_at IS NULL;

-- Manages active user's sessions and attached metadata.
CREATE TABLE account_sessions
(
    -- Unique token for each session, per account.
    token_seq  UUID      NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id UUID REFERENCES accounts (id) ON DELETE CASCADE,
    -- Two-character region identifier (e.g., "US", "EU").
    region_id  CHAR(2)   NOT NULL DEFAULT 'A0',

    -- Each token sequence must be unique per account.
    CONSTRAINT account_sessions_pkey PRIMARY KEY (account_id, token_seq),
    -- Region identifier must be alphanumeric and exactly 2 characters.
    CONSTRAINT account_sessions_region_alphanumeric CHECK (region_id ~ '^[A-Z0-9]{2}$'),

    -- Security-related session information.
    ip_address INET      NOT NULL,
    user_agent TEXT      NOT NULL,

    -- Timestamps for tracking the row's lifecycle.
    issued_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    expired_at TIMESTAMP NOT NULL DEFAULT current_timestamp + INTERVAL '1 day' * 7,
    deleted_at TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_sessions_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_sessions_deleted_after_issued CHECK (deleted_at IS NULL OR deleted_at >= issued_at)
);

-- Optimizes lookup for active sessions by account & token.
CREATE INDEX account_sessions_active_idx
    ON account_sessions (account_id, token_seq)
    WHERE deleted_at IS NULL;

-- Tracks account's read (anything) and write (anything) privileges.
CREATE TABLE account_permissions
(
    -- Reference to the associated account.
    account_id    UUID PRIMARY KEY REFERENCES accounts (id) ON DELETE CASCADE,

    -- Universal permissions flags (read & write rights).
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

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('account_permissions');

-- Optimizes lookup for active permissions by the account identifier.
CREATE INDEX account_permissions_idx
    ON account_permissions (account_id)
    WHERE deleted_at IS NOT NULL;

-- Defines an enumeration for action types confirmed via email.
CREATE TYPE EMAIL_ACTION AS ENUM ('confirm_email', 'reset_password');

-- Tracks account's tokens used for (email-confirmed) actions.
CREATE TABLE account_tokens
(
    -- Unique action token per email action.
    action_token  UUID         NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id    UUID REFERENCES accounts (id) ON DELETE CASCADE,

    -- Each token sequence must be unique per account.
    CONSTRAINT account_tokens_pkey PRIMARY KEY (account_id, action_token),

    -- Receiver's email address and action type.
    email_address TEXT         NOT NULL,
    action_type   EMAIL_ACTION NOT NULL,
    -- TODO: Add metadata e.g. email to change to or password reset by.
    -- TODO: Move metadata to metadata?

    -- Timestamps for tracking the row's lifecycle.
    issued_at     TIMESTAMP    NOT NULL DEFAULT current_timestamp,
    expired_at    TIMESTAMP    NOT NULL DEFAULT current_timestamp + INTERVAL '7 days',
    used_at       TIMESTAMP             DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_tokens_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_tokens_used_after_issued CHECK (used_at IS NULL OR used_at >= issued_at)
);

-- Optimizes lookup for active email tokens by the account identifier.
CREATE INDEX account_tokens_idx
    ON account_tokens (account_id, action_token)
    WHERE used_at IS NULL;
