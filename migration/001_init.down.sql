DROP TABLE IF EXISTS accounts CASCADE;
DROP TABLE IF EXISTS account_sessions CASCADE;
DROP TABLE IF EXISTS account_permissions CASCADE;
DROP TABLE IF EXISTS account_emails CASCADE;

DROP TABLE IF EXISTS projects CASCADE;
DROP TABLE IF EXISTS project_members CASCADE;
DROP TABLE IF EXISTS project_invites CASCADE;
DROP TABLE IF EXISTS project_webhooks CASCADE;

DROP TABLE IF EXISTS workflows CASCADE;
DROP TABLE IF EXISTS workflow_schedules CASCADE;
DROP TABLE IF EXISTS workflow_webhooks CASCADE;
DROP TABLE IF EXISTS workflow_executions CASCADE;

DROP FUNCTION IF EXISTS refresh_update_at;

DROP TYPE IF EXISTS EMAIL_TYPE;
DROP TYPE IF EXISTS INVITE_STATUS;
