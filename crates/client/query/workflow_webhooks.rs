//! Data layer for workflow webhooks management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::workflows::WorkflowViewOutput;
use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workflow_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowWebhookCreateInput {
    pub workflow_id: Uuid,
    pub webhook_id: Uuid,
}

/// Creates all workflow webhooks associated with a workflow.
///
/// Executed once the associated workflow is updated.
///
/// # Tables
///
/// - workflow_webhooks
pub async fn create_all_workflow_webhooks(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
    form_webhook_ids: Vec<Uuid>,
) -> DatabaseResult<()> {
    use schema::workflow_webhooks::dsl::*;

    let new_workflow_webhooks: Vec<_> = form_webhook_ids
        .into_iter()
        .map(|id| WorkflowWebhookCreateInput {
            workflow_id: form_workflow_id,
            webhook_id: id,
        })
        .collect();

    insert_into(workflow_webhooks)
        .values(&new_workflow_webhooks)
        .execute(conn)
        .await?;

    Ok(())
}

/// Returns all workflows that use the webhook.
///
/// # Tables
///
/// - workflows
/// - workflow_webhooks
pub async fn view_workflows_by_webhook(
    conn: &mut AsyncPgConnection,
    webhook_id: Uuid,
) -> DatabaseResult<Vec<WorkflowViewOutput>> {
    use schema::workflow_webhooks::dsl as wfh_dsl;
    use schema::workflows::dsl as wf_dsl;

    let query = wf_dsl::workflows
        .inner_join(wfh_dsl::workflow_webhooks.on(wf_dsl::workspace_id.eq(wfh_dsl::workflow_id)))
        .filter(wfh_dsl::webhook_id.eq(webhook_id))
        .select(WorkflowViewOutput::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}

/// Deletes all workflow webhooks associated with a workflow.
///
/// Executed when the associated workflow is updated.
///
/// # Tables
///
/// - workflow_webhooks
pub async fn delete_all_workflow_webhooks(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workflow_webhooks::dsl::*;

    let filter_cond = workflow_webhooks.filter(workflow_id.eq(form_workflow_id));
    let _query = delete(filter_cond).execute(conn).await?;

    Ok(())
}
