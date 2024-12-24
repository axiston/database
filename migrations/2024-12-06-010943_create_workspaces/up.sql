-- Creates the `projects` table to manage project information.
CREATE TABLE projects
(
    -- Unique identifier for each project (used as a public resource).
    id           UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- User-provided project name.
    display_name TEXT      NOT NULL DEFAULT 'Untitled',
    -- User-provided project metadata (description, tags, etc).
    project_meta JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Prevents the empty project name.
    CONSTRAINT projects_non_empty_display_name CHECK (display_name <> ''),
    -- Limits the size of the project metadata JSONB field.
    CONSTRAINT projects_workflow_meta_limit CHECK (length(project_meta::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT projects_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT projects_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT projects_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('projects');

-- Creates `project_members` table to manage project memberships.
CREATE TABLE project_members
(
    -- Reference to the associated project.
    project_id UUID      NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    -- Reference to the associated account.
    account_id UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Ensures each member and project pair is unique.
    CONSTRAINT project_members_pkey PRIMARY KEY (project_id, account_id),

    -- Flags for visibility and project priority.
    show_order INT       NOT NULL DEFAULT 0,
    is_pinned  BOOLEAN   NOT NULL DEFAULT FALSE,
    is_hidden  BOOLEAN   NOT NULL DEFAULT FALSE,

    -- Same as `account_id` if the user joined as project owner.
    created_by UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Same as `account_id` if the user left on their own.
    updated_by UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking member's record lifecycle.
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT project_members_updated_after_created CHECK (updated_at >= created_at)
);

-- Optimizes lookup for members by account.
CREATE INDEX project_members_account_id_idx ON project_members (account_id);
-- Optimizes lookup for members by project.
CREATE INDEX project_members_project_id_idx ON project_members (project_id);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('project_members');

-- Defines an ENUM type for invite status, with possible values:
-- 'pending' (default), 'accepted', 'declined', and 'canceled'.
CREATE TYPE INVITE_STATUS AS ENUM ('pending', 'accepted', 'declined', 'canceled');

-- Create `project_invites` table to manage project invitations.
CREATE TABLE project_invites
(
    -- Reference to the associated project (used as a public resource).
    project_id UUID          NOT NULL REFERENCES projects (id) ON DELETE CASCADE,
    -- Unique identifier for each invite per project (used as a public resource).
    invite_id  UUID          NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Ensures each project and invite pair is unique.
    CONSTRAINT project_invites_pkey PRIMARY KEY (project_id, invite_id),

    -- Current status of the invite (pending, accepted, declined, or canceled).
    status     INVITE_STATUS NOT NULL DEFAULT 'pending',

    -- Can't be the same as `account_id`.
    created_by UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Same as `account_id` if the user declined, other if canceled.
    updated_by UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at TIMESTAMP     NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP     NOT NULL DEFAULT current_timestamp,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT project_invites_updated_after_created CHECK (updated_at >= created_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('project_invites');

-- Optimizes lookup for invites by account.
CREATE INDEX project_invites_account_id_idx ON project_invites (account_id);
-- Optimizes lookup for invites by project.
CREATE INDEX project_invites_project_id_idx ON project_invites (project_id);

-- Creates `project_permissions` table to manage project permissions.
CREATE TABLE project_permissions
(
    -- Reference to the associated account.
    account_id UUID      NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Reference to the associated project.
    project_id UUID      NOT NULL REFERENCES projects (id) ON DELETE CASCADE,

    CONSTRAINT project_permissions_pkey PRIMARY KEY (account_id, project_id),

    -- Timestamps for tracking the row's lifecycle.
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT project_permissions_updated_after_created CHECK (
        updated_at >= created_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('project_permissions');

-- Creates `project_schedules` table to manage project schedules.
CREATE TABLE project_schedules
(
    -- Unique identifier for each schedule (used as a public resource).
    id         UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- Timestamps for tracking the row's lifecycle.
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT project_schedules_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT project_schedules_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT project_schedules_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('project_schedules');

-- Creates `project_webhooks` table to manage project webhooks.
CREATE TABLE project_webhooks
(
    -- Unique identifier for each webhook (used as a public resource).
    id           UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Reference to the associated project.
    project_id   UUID      NOT NULL REFERENCES projects (id) ON DELETE CASCADE,

    success_code INTEGER   NOT NULL DEFAULT 200,
    failure_code INTEGER   NOT NULL DEFAULT 400,
    wait_output  BOOL      NOT NULL DEFAULT FALSE,

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT project_webhooks_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT project_webhooks_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT project_webhooks_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('project_webhooks');

-- CREATE TABLE events
-- (
--     -- project_invites changes
--     -- project_members changes
--     -- project_webhooks changes
--     -- workflows changes
-- );
