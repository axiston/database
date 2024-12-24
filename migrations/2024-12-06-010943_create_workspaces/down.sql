-- Drop `project_webhooks` table.
DROP TABLE IF EXISTS project_webhooks;

-- Drop `project_permissions` table.
DROP TABLE IF EXISTS project_permissions;

-- Drop `project_invites` table and associated objects.
DROP INDEX IF EXISTS project_invites_account_id_idx;
DROP INDEX IF EXISTS project_invites_project_id_idx;
DROP TABLE IF EXISTS project_invites;
DROP TYPE IF EXISTS INVITE_STATUS;

-- Drop `project_members` table and associated objects.
DROP INDEX IF EXISTS project_members_account_id_idx;
DROP INDEX IF EXISTS project_members_project_id_idx;
DROP TABLE IF EXISTS project_members;

-- Drop `projects` table.
DROP TABLE IF EXISTS projects;
