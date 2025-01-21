-- Manage general workflow metadata.
CREATE TABLE workflows
(
    -- Unique identifier for each workflow, used as a public resource.
    id             UUID PRIMARY KEY   DEFAULT gen_random_uuid(),
    -- Reference to the workspace that owns this workflow.
    workspace_id   UUID REFERENCES workspaces (id) ON DELETE CASCADE,
    -- User-provided unique workflow name within the workspace.
    display_name   TEXT      NOT NULL DEFAULT 'Untitled',

    -- User-defined metadata (e.g., description, tags, etc.).
    metadata_props JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Ensures the workflow name is unique within a workspace.
    CONSTRAINT workflows_unique_display_name UNIQUE (workspace_id, display_name),
    -- Prevents empty workflow names.
    CONSTRAINT workflows_non_empty_display_name CHECK (display_name <> ''),
    -- Restricts the size of the metadata field.
    CONSTRAINT workflows_metadata_props_limit CHECK (length(metadata_props::TEXT) <= 2048),

    -- Serialized representation of the workflow input graph.
    input_graph    JSONB     NOT NULL DEFAULT '{}'::JSONB,
    -- Runtime information for workflow execution (e.g., version, errors).
    runtime_meta   JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Restricts the size of the input graph.
    CONSTRAINT workflows_input_graph_limit CHECK (length(input_graph::TEXT) <= 4096),
    -- Restricts the size of runtime metadata.
    CONSTRAINT workflows_runtime_meta_limit CHECK (length(runtime_meta::TEXT) <= 2048),

    -- Timestamps for tracking the row's lifecycle.
    created_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMP NOT NULL DEFAULT current_timestamp,
    deleted_at     TIMESTAMP          DEFAULT NULL,

    -- Constraints to ensure proper lifecycle management.
    CONSTRAINT updated_after_created CHECK (updated_at >= created_at),
    CONSTRAINT deleted_after_created CHECK (deleted_at IS NULL OR deleted_at >= created_at),
    CONSTRAINT deleted_after_updated CHECK (deleted_at IS NULL OR deleted_at >= updated_at)
);

-- Automatically updates the `updated_at` timestamp on any row's update.
SELECT manage_updated_at('workflows');

-- Optimizes lookup for workflows within a workspace.
CREATE INDEX workflows_workspace_idx
    ON workflows (workspace_id)
    WHERE deleted_at IS NULL;

-- Manages workflow schedules.
CREATE TABLE workflow_schedules
(
    -- Reference to the associated workflow, used as a public resource.
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each schedule associated with a workflow.
    schedule_id UUID      NOT NULL REFERENCES workspace_schedules (id) ON DELETE CASCADE,

    -- Ensures each workflow-schedule pair is unique.
    CONSTRAINT workflow_schedules_pkey PRIMARY KEY (workflow_id, schedule_id),

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- Manages workflow webhooks.
CREATE TABLE workflow_webhooks
(
    -- Reference to the associated workflow, used as a public resource.
    workflow_id UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each webhook associated with a workflow.
    webhook_id  UUID      NOT NULL REFERENCES workspace_webhooks (id) ON DELETE CASCADE,

    -- Ensures each workflow-webhook pair is unique.
    CONSTRAINT workflow_webhooks_pkey PRIMARY KEY (workflow_id, webhook_id),

    -- Timestamps for tracking the row's lifecycle.
    created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp
);

-- Manages workflow executions.
CREATE TABLE workflow_executions
(
    -- Reference to the associated workflow, used as a public resource.
    workflow_id  UUID      NOT NULL REFERENCES workflows (id) ON DELETE CASCADE,
    -- Unique identifier for each execution of a workflow.
    execution_id UUID      NOT NULL DEFAULT gen_random_uuid(),

    -- Ensures each workflow-execution pair is unique.
    CONSTRAINT workflow_executions_pkey PRIMARY KEY (workflow_id, execution_id),

    -- Serialized representation of the workflow output graph.
    output_graph JSONB     NOT NULL DEFAULT '{}'::JSONB,
    -- Runtime information for the execution (e.g., version, errors).
    runtime_meta JSONB     NOT NULL DEFAULT '{}'::JSONB,

    -- Restricts the size of the output graph.
    CONSTRAINT workflow_executions_output_graph_limit CHECK (length(output_graph::TEXT) <= 4096),
    -- Restricts the size of runtime metadata.
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
