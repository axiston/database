-- Creates the `accounts` table to manage user account information.
CREATE TABLE accounts
(
    -- Unique identifier for each account (used as a public resource).
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- User-provided account name, typically a full name.
    display_name  TEXT      NOT NULL,
    -- User-provided unique email address (ignores deactivated accounts).
    email_address TEXT      NOT NULL,
    -- Hashed version of the user-provided password.
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
CREATE UNIQUE INDEX accounts_unique_email_address_idx ON accounts (email_address)
    WHERE deleted_at IS NULL;
-- Optimizes lookup for active accounts by email and password.
CREATE INDEX accounts_local_credentials_idx ON accounts (email_address, password_hash)
    WHERE deleted_at IS NULL;

-- Creates the `account_sessions` table to manage user sessions.
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
CREATE INDEX account_sessions_only_active_idx ON account_sessions (account_id, token_seq)
    WHERE deleted_at IS NULL;

-- Creates the `account_permissions` table to track user privileges.
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
-- Optimizes lookup for active permissions with absolute privileges.
CREATE INDEX account_permissions_absolute_idx ON account_permissions (account_id)
    WHERE deleted_at IS NOT NULL AND nocheck_read IS TRUE AND nocheck_write IS TRUE;

-- Defines an enumeration for email (action) types.
CREATE TYPE EMAIL_TYPE AS ENUM ('confirm_email', 'update_email', 'reset_password');

-- Creates the `account_actions` table to track email actions.
CREATE TABLE account_actions
(
    -- Unique action token per email action.
    action_token  UUID PRIMARY KEY    DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id    UUID REFERENCES accounts (id) ON DELETE CASCADE,

    -- Email address and action details.
    email_address TEXT       NOT NULL,
    action_type   EMAIL_TYPE NOT NULL,

    -- Timestamps for tracking the row's lifecycle.
    issued_at     TIMESTAMP  NOT NULL DEFAULT current_timestamp,
    expired_at    TIMESTAMP  NOT NULL DEFAULT current_timestamp + INTERVAL '1 day' * 7,
    used_at       TIMESTAMP           DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT account_actions_expired_after_issued CHECK (expired_at >= issued_at),
    CONSTRAINT account_actions_used_after_issued CHECK (used_at IS NULL OR used_at >= issued_at)
);

-- Optimizes lookup for active email tokens by account.
CREATE INDEX account_actions_active_idx ON account_actions (account_id, action_token)
    WHERE used_at IS NULL;
