//! Data layer for workflow schedules management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::workflow_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowScheduleCreateInput {
    pub workflow_id: Uuid,
    pub schedule_id: Uuid,
}

/// Creates all workflow schedules associated with a workflow.
///
/// Executed once the associated workflow is updated.
///
/// # Tables
///
/// - workflow_schedules
pub async fn create_all_workflow_schedules(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
    form_schedule_ids: Vec<Uuid>,
) -> DatabaseResult<()> {
    use schema::workflow_schedules::dsl::*;

    let new_workflow_schedules: Vec<_> = form_schedule_ids
        .into_iter()
        .map(|id| WorkflowScheduleCreateInput {
            workflow_id: form_workflow_id,
            schedule_id: id,
        })
        .collect();

    insert_into(workflow_schedules)
        .values(&new_workflow_schedules)
        .execute(conn)
        .await?;

    Ok(())
}

/// Deletes all workflow schedules associated with a workflow.
///
/// Executed when the associated workflow is updated.
///
/// # Tables
///
/// - workflow_schedules
pub async fn delete_all_workflow_schedules(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workflow_schedules::dsl::*;

    let filter_cond = workflow_schedules.filter(workflow_id.eq(form_workflow_id));
    let _query = delete(filter_cond).execute(conn).await?;

    Ok(())
}
