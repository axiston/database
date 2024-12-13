-- Creates the `workflows` table to manage workflows.
CREATE TABLE workflows
(
    -- Unique identifier for each workflow (used as a public resource).
    id           UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    project_id   UUID REFERENCES projects (id) ON DELETE CASCADE,

    -- User-provided unique workflow name within each project.
    display_name TEXT      NOT NULL DEFAULT 'Untitled',
    -- Tags, descriptions, and other optional properties.
    properties   JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures workflow name uniqueness per project.
    CONSTRAINT workflows_unique_display_name UNIQUE (project_id, display_name),
    -- Ensures workflow name is not empty.
    CONSTRAINT workflows_non_empty_display_name CHECK (display_name <> ''),
    -- Limits the size of the meta JSONB field.
    CONSTRAINT workflows_properties_limit CHECK (length(properties::TEXT) <= 2048),

    -- Deserialized, minified and serialized workflow.
    input_graph  JSONB     NOT NULL DEFAULT '{}'::JSONB,
    -- Compiler information, including version and error counts.
    runtime_meta JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Limits the size of the input workflow graph.
    CONSTRAINT workflows_input_graph_limit CHECK (length(input_graph::TEXT) <= 4096),
    -- Limits the size of the runtime metadata.
    CONSTRAINT workflows_runtime_meta_limit CHECK (length(runtime_meta::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workflows');
-- Optimizes lookup for workflows by project.
CREATE INDEX workflows_project_idx ON workflows (project_id) WHERE deleted_at IS NULL;

-- Creates the `workflow_schedules` table to manage workflows schedules.
CREATE TABLE workflow_schedules
(
    -- Reference to the associated workflow (used as a public resource).
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each schedule per workflow (used as a public resource).
    schedule_id UUID      NOT NULL DEFAULT gen_random_uuid(),

    -- Ensures each workflow and schedule pair is unique.
    CONSTRAINT workflow_schedules_pkey PRIMARY KEY (workflow_id, schedule_id),

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_schedules_deleted_after_created CHECK (deleted_at >= created_at)
);

-- Creates the `workflow_webhooks` table to manage workflows webhooks.
CREATE TABLE workflow_webhooks
(
    -- Reference to the associated workflow (used as a public resource).
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each webhook per workflow (used as a public resource).
    webhook_id  UUID               DEFAULT gen_random_uuid(),

    -- Ensures each workflow and webhook pair is unique.
    CONSTRAINT workflow_webhooks_pkey PRIMARY KEY (workflow_id, webhook_id),

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at  TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_webhooks_deleted_after_created CHECK (deleted_at >= created_at)
);

-- Creates the `workflow_executions` table to manage workflows executions.
-- `created_at` also functions as an execution start timestamp.
-- `updated_at` also functions as an execution end timestamp.
CREATE TABLE workflow_executions
(
    -- Reference to the associated workflow (used as a public resource).
    workflow_id  UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each execution per workflow (used as a public resource).
    execution_id UUID      NOT NULL DEFAULT gen_random_uuid(),

    -- Ensures each workflow and execution pair is unique.
    CONSTRAINT workflow_executions_pkey PRIMARY KEY (workflow_id, execution_id),

    -- Deserialized, minified and serialized again workflow output.
    output_graph JSONB     NOT NULL DEFAULT '{}'::JSONB,
    -- Compiler information, including version and error counts.
    runtime_meta JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Limits the size of the input workflow graph.
    CONSTRAINT workflow_executions_output_graph_limit CHECK (length(output_graph::TEXT) <= 4096),
    -- Limits the size of the runtime metadata.
    CONSTRAINT workflow_executions_runtime_meta_limit CHECK (length(runtime_meta::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at   TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at   TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT workflow_executions_updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT workflow_executions_deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT workflow_executions_deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);
