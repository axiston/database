-- Manages general workspace metadata.
CREATE TABLE workspaces
(
    -- Unique identifier for each workspace (used as a public resource).
    id             UUID PRIMARY KEY   DEFAULT gen_random_uuid(),

    -- User-provided workspace name.
    display_name   TEXT      NOT NULL DEFAULT 'Untitled',
    -- User-provided workspace metadata (description, tags, etc).
    metadata_props JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Prevents the empty workspace name.
    CONSTRAINT workspaces_non_empty_display_name CHECK (display_name <> ''),
    -- Limits the size of the workspace metadata JSONB field.
    CONSTRAINT workspaces_metadata_props_limit CHECK (length(metadata_props::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at     TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workspaces_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspaces_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspaces_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workspaces');

-- TODO.
CREATE TYPE PERMISSION_ROLE AS ENUM ('owner', 'member');

-- Manages workspace memberships and display order.
CREATE TABLE workspace_members
(
    -- Reference to the associated workspace.
    workspace_id UUID            NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- Reference to the associated account.
    account_id   UUID            NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Permission role of the associated account.
    account_role PERMISSION_ROLE NOT NULL DEFAULT 'member',

    -- Ensures each member and workspace pair is unique.
    CONSTRAINT workspace_members_pkey PRIMARY KEY (workspace_id, account_id),

    -- Flags for visibility and workspace priority.
    show_order   INT             NOT NULL DEFAULT 0,
    is_pinned    BOOLEAN         NOT NULL DEFAULT FALSE,
    is_hidden    BOOLEAN         NOT NULL DEFAULT FALSE,

    -- Same as `account_id` if the user joined as workspace owner.
    created_by   UUID            NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Same as `account_id` if the user left on their own.
    updated_by   UUID            NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking member's record lifecycle.
    created_at   TIMESTAMP       NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP       NOT NULL DEFAULT current_timestamp,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workspace_members_updated_after_created CHECK (updated_at >= created_at)
);

-- Optimizes lookup for members by the account identifier.
CREATE INDEX workspace_members_account_id_idx
    ON workspace_members (account_id);

-- Optimizes lookup for members by the workspace identifier.
CREATE INDEX workspace_members_workspace_id_idx
    ON workspace_members (workspace_id);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workspace_members');

-- Defines an ENUM type for invite status, with possible values:
-- 'pending' (default), 'accepted', 'declined', and 'canceled'.
CREATE TYPE INVITE_STATUS AS ENUM ('pending', 'accepted', 'declined', 'canceled');

-- Manages workspace invitations.
CREATE TABLE workspace_invites
(
    -- Reference to the associated workspace (used as a public resource).
    workspace_id  UUID          NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- Unique identifier for each invite per workspace (used as a public resource).
    invite_id     UUID          NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated account.
    account_id    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Current status of the invite (pending, accepted, declined, or canceled).
    invite_status INVITE_STATUS NOT NULL DEFAULT 'pending',

    -- Ensures each workspace and invite pair is unique.
    CONSTRAINT workspace_invites_pkey PRIMARY KEY (workspace_id, invite_id),

    -- Can't be the same as `account_id`.
    created_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Same as `account_id` if the user declined, other if canceled.
    updated_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMP     NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP     NOT NULL DEFAULT current_timestamp,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workspace_invites_updated_after_created CHECK (updated_at >= created_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workspace_invites');

ALTER TYPE EMAIL_ACTION ADD VALUE 'pending_invite';

-- Optimizes lookup for invites by the account identifier.
CREATE INDEX workspace_invites_account_id_idx
    ON workspace_invites (account_id);

-- Optimizes lookup for invites by the workspace identifier.
CREATE INDEX workspace_invites_workspace_id_idx
    ON workspace_invites (workspace_id);

-- Manages workspace schedules.
CREATE TABLE workspace_schedules
(
    -- Unique identifier for each schedule (used as a public resource).
    id             UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Reference to the associated workspace.
    workspace_id   UUID      NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- User-provided workspace metadata (cron, description, tags, etc).
    metadata_props JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Limits the size of the workspace metadata JSONB field.
    CONSTRAINT workspace_schedules_metadata_props_limit CHECK (length(metadata_props::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at     TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workspace_schedules_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspace_schedules_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspace_schedules_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workspace_schedules');

-- Manages workspace webhooks.
CREATE TABLE workspace_webhooks
(
    -- Unique identifier for each webhook (used as a public resource).
    id             UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Reference to the associated workspace.
    workspace_id   UUID      NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- User-provided workspace metadata (cron, description, tags, etc).
    metadata_props JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Limits the size of the workspace metadata JSONB field.
    CONSTRAINT workspace_webhooks_metadata_props_limit CHECK (length(metadata_props::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at     TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workspace_webhooks_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspace_webhooks_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspace_webhooks_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workspace_webhooks');


-- success_code INTEGER   NOT NULL DEFAULT 200,
--     failure_code INTEGER   NOT NULL DEFAULT 400,
--     wait_output  BOOL      NOT NULL DEFAULT FALSE,
