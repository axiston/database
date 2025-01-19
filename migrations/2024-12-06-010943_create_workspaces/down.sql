-- Drop `workspace_webhooks` table.
DROP TABLE IF EXISTS workspace_webhooks;
DROP TABLE IF EXISTS workspace_schedules;

-- Drop `workspace_invites` table and associated objects.
DROP INDEX IF EXISTS workspace_invites_account_id_idx;
DROP INDEX IF EXISTS workspace_invites_workspace_id_idx;
DROP TABLE IF EXISTS workspace_invites;
DROP TYPE IF EXISTS INVITE_STATUS;

-- Drop `workspace_members` table and associated objects.
DROP INDEX IF EXISTS workspace_members_account_id_idx;
DROP INDEX IF EXISTS workspace_members_workspace_id_idx;
DROP TABLE IF EXISTS workspace_members;
DROP TYPE IF EXISTS PERMISSION_ROLE;

-- Drop `workspaces` table.
DROP TABLE IF EXISTS workspaces;
