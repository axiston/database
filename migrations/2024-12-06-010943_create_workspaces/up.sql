-- Workspace management with metadata tracking.
CREATE TABLE workspaces
(
    -- Unique identifier for each workspace, used as a public resource.
    id           UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    -- Name of the workspace, provided by the user.
    display_name TEXT        NOT NULL DEFAULT 'Untitled',
    -- Additional workspace metadata, stored in JSON format (e.g., description, tags).
    metadata     JSONB       NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures the workspace name is not empty.
    CONSTRAINT workspaces_non_empty_display_name CHECK (display_name <> ''),
    -- Restricts the size of the metadata field to avoid excessively large JSON data.
    CONSTRAINT workspaces_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMPTZ          DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT workspaces_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspaces_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspaces_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('workspaces');

-- Represents the possible roles a user can have in a workspace.
CREATE TYPE PROJECT_ROLE AS ENUM ('owner', 'member');

-- Memberships and workspace permissions management.
CREATE TABLE workspace_members
(
    -- Reference to the workspace this membership belongs to.
    workspace_id UUID         NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- Reference to the account of the member.
    account_id   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Role of the member within the workspace, defaulting to 'member'.
    account_role PROJECT_ROLE NOT NULL DEFAULT 'member',

    -- Ensures each workspace-account pair is unique.
    CONSTRAINT workspace_members_pkey PRIMARY KEY (workspace_id, account_id),

    -- Defines the display order of the workspace for the user.
    show_order   INT          NOT NULL DEFAULT 0,
    -- Indicates if the workspace is pinned for easier access.
    is_pinned    BOOLEAN      NOT NULL DEFAULT FALSE,
    -- Indicates if the workspace is hidden from the user's dashboard.
    is_hidden    BOOLEAN      NOT NULL DEFAULT FALSE,

    -- Tracks who created this membership record.
    created_by   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Tracks who last updated this membership record.
    updated_by   UUID         NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking member's record lifecycle.
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT workspace_members_updated_after_created CHECK (updated_at >= created_at)
);

-- Optimizes lookup for members by the account ID.
CREATE INDEX workspace_members_account_id_idx
    ON workspace_members (account_id);

-- Optimizes lookup for members by the workspace ID.
CREATE INDEX workspace_members_workspace_id_idx
    ON workspace_members (workspace_id);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('workspace_members');

-- Defines the possible statuses for workspace invitations.
-- 'pending' (default), 'accepted', 'declined', and 'canceled'.
CREATE TYPE INVITE_STATUS AS ENUM ('pending', 'accepted', 'declined', 'canceled');

-- Manages invitations sent to users to join a workspace.
CREATE TABLE workspace_invites
(
    -- Unique identifier for each invitation, used as a public resource.
    invite_id     UUID          NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the workspace the invitation belongs to.
    workspace_id  UUID          NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,
    -- Reference to the account being invited.
    account_id    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Current status of the invitation, defaulting to 'pending'.
    invite_status INVITE_STATUS NOT NULL DEFAULT 'pending',

    -- Ensures each workspace-invite pair is unique.
    CONSTRAINT workspace_invites_pkey PRIMARY KEY (workspace_id, invite_id),

    -- Tracks who created the invitation (e.g., the inviter).
    created_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
    -- Tracks who last updated the invitation (e.g., the invitee, the inviter, or an admin).
    updated_by    UUID          NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMPTZ   NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMPTZ   NOT NULL DEFAULT current_timestamp,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT workspace_invites_updated_after_created CHECK (updated_at >= created_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('workspace_invites');

-- Adds a new value to token actions to track pending invites.
ALTER TYPE TOKEN_ACTION ADD VALUE 'pending_invite';

CREATE INDEX workspace_project_invites_account_id_idx
    ON workspace_invites (account_id);

CREATE INDEX workspace_project_invites_workspace_project_id_idx
    ON workspace_invites (workspace_id);

-- Manages schedules associated with workspaces.
CREATE TABLE workspace_schedules
(
    -- Unique identifier for each schedule, used as a public resource.
    id              UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated workspace.
    workspace_id    UUID             NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,

    -- Defines the schedule update interval (in seconds).
    update_interval INTEGER          NOT NULL DEFAULT 3600,
    -- User-provided workspace metadata (cron, description, tags, etc).
    metadata        JSONB            NOT NULL DEFAULT '{}'::JSONB,

    -- Restricts the update interval to at least 1 second.
    CONSTRAINT workspace_schedules_update_interval_non_zero CHECK ( update_interval > 0 ),
    -- Restricts the size of the metadata field to avoid excessively large JSON data.
    CONSTRAINT workspace_schedules_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at      TIMESTAMPTZ      NOT NULL DEFAULT current_timestamp,
    updated_at      TIMESTAMPTZ      NOT NULL DEFAULT current_timestamp,
    deleted_at      TIMESTAMPTZ               DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT workspace_schedules_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspace_schedules_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspace_schedules_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('workspace_schedules');

-- Manages webhooks associated with workspaces.
CREATE TABLE workspace_webhooks
(
    -- Unique identifier for each webhook, used as a public resource.
    id           UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    -- Reference to the associated workspace.
    workspace_id UUID             NOT NULL REFERENCES workspaces (id) ON DELETE CASCADE,

    -- User-provided workspace metadata (cron, description, tags, etc).
    metadata     JSONB            NOT NULL DEFAULT '{}'::JSONB,

    -- Restricts the size of the metadata field to avoid excessively large JSON data.
    CONSTRAINT workspace_webhooks_metadata_limit CHECK (length(metadata::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMPTZ      NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMPTZ               DEFAULT NULL,

    -- Integrity checks to maintain chronological consistency.
    CONSTRAINT workspace_webhooks_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workspace_webhooks_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workspace_webhooks_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates modification timestamp.
SELECT manage_updated_at('workspace_webhooks');
