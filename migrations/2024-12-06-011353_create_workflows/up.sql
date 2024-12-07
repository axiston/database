-- Creates the `workflows` table to manage workflows.
CREATE TABLE workflows
(
    -- Unique identifier for a workflow, used as a public resource.
    id            UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    project_id    UUID REFERENCES projects (id) ON DELETE CASCADE,

    -- User-provided unique workflow name within each project.
    display_name  TEXT      NOT NULL DEFAULT 'Untitled',
    -- Tags, descriptions, and other optional properties.
    properties    JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures workflow name uniqueness per project.
    CONSTRAINT workflows_unique_display_name UNIQUE (project_id, display_name),
    -- Ensures workflow name is not empty.
    CONSTRAINT workflows_non_empty_display_name CHECK (display_name <> ''),
    -- Limits the size of the meta JSONB field.
    CONSTRAINT workflows_meta_limit CHECK (length(properties::TEXT) <= 2048),

    -- Deserialized, minified and serialized workflow.
    input_graph   JSONB     NOT NULL DEFAULT '{}'::JSONB,
    -- Compiler information, including version and error counts.
    compiler_meta JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Timestamps for tracking the row's lifecycle.
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at    TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workflows');

-- Creates the `workflow_schedules` table to manage workflows schedules.
CREATE TABLE workflow_schedules
(
    -- Unique identifier for a workflow schedules, used as a public resource.
    id          UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Unique identifier for a workflow, used as a public resource.
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_schedules_deleted_after_created CHECK ( deleted_at >= created_at )
);

-- Creates the `workflow_webhooks` table to manage workflows webhooks.
CREATE TABLE workflow_webhooks
(
    -- Unique identifier for a workflow webhooks, used as a public resource.
    id          UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Unique identifier for a workflow, used as a public resource.
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_webhooks_deleted_after_created CHECK ( deleted_at >= created_at )
);

-- Creates the `workflow_executions` table to manage workflows executions.
CREATE TABLE workflow_executions
(
    -- Unique identifier for a workflow executions, used as a public resource.
    id          UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Unique identifier for a workflow, used as a public resource.
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_executions_deleted_after_created CHECK ( deleted_at >= created_at )
);
