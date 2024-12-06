-- Drops the `account_emails` and associated objects.
DROP TABLE IF EXISTS account_emails;
DROP TYPE IF EXISTS EMAIL_TYPE;

-- Drop the `account_permissions` and `account_sessions` table.
DROP TABLE IF EXISTS account_permissions;
DROP TABLE IF EXISTS account_sessions;

-- Drop the `accounts` table and associated objects.
DROP INDEX IF EXISTS accounts_local_credentials_idx;
DROP INDEX IF EXISTS accounts_unique_email_address_idx;
DROP TABLE IF EXISTS accounts;
