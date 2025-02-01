-- Drops account action tokens and associated objects.
DROP TABLE IF EXISTS account_tokens;
DROP TYPE IF EXISTS TOKEN_ACTION;

-- Drops user permissions, sessions and associated objects.
DROP INDEX IF EXISTS account_permissions_absolute_idx;
DROP TABLE IF EXISTS account_permissions;
DROP INDEX IF EXISTS account_sessions_only_active_idx;
DROP TABLE IF EXISTS account_sessions;

-- Drops the user accounts and associated objects.
DROP INDEX IF EXISTS accounts_local_credentials_idx;
DROP INDEX IF EXISTS accounts_unique_email_address_idx;
DROP TABLE IF EXISTS accounts;
