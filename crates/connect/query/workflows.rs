use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{Database, DatabaseResult};

/// Queries for the `workflows` table.
pub trait WorkflowsExt {
    /// Creates a new workflow.
    ///
    /// # Tables
    ///
    /// - workflows
    async fn create_workflow(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<Uuid>;

    /// Updates workflow's graph.
    ///
    /// # Tables
    ///
    /// - workflows
    async fn update_workflow_graph(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// Updates workflow's metadata.
    ///
    /// # Tables
    ///
    /// - workflows
    async fn update_workflow_metadata(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// Deletes an existing workflow.
    ///
    /// # Tables
    ///
    /// - workflows
    async fn delete_workflow(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;
}

// impl WorkflowsExt for Database {}
