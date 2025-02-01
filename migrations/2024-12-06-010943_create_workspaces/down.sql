-- Drops workspace webhooks, schedules and associated objects.
DROP TABLE IF EXISTS workspace_webhooks;
DROP TABLE IF EXISTS workspace_schedules;

-- Drops workspace invites and associated objects.
DROP INDEX IF EXISTS workspace_invites_account_id_idx;
DROP INDEX IF EXISTS workspace_invites_workspace_id_idx;
DROP TABLE IF EXISTS workspace_invites;
DROP TYPE IF EXISTS INVITE_STATUS;

-- Drops workspace members and associated objects.
DROP INDEX IF EXISTS workspace_members_account_id_idx;
DROP INDEX IF EXISTS workspace_members_workspace_id_idx;
DROP TABLE IF EXISTS workspace_members;
DROP TYPE IF EXISTS PROJECT_ROLE;

-- Drops workspaces and associated objects.
DROP TABLE IF EXISTS workspaces;
