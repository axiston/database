//! Data layer for workflow executions management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dsl::*;
use crate::{DatabaseResult, QueryOrderBy};

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::workflow_executions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionCreateInput {
    pub workflow_id: Uuid,
    pub output_graph: Value,
    pub rt_metadata: Value,

    pub started_at: OffsetDateTime,
    pub ended_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::workflow_executions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionCreateOutput {
    pub execution_id: Uuid,
}

/// Creates the new workflow execution and returns its ID.
///
/// # Tables
///
/// - workflow_executions
pub async fn create_workflow_execution(
    conn: &mut AsyncPgConnection,
    form: WorkflowExecutionCreateInput,
) -> DatabaseResult<WorkflowExecutionCreateOutput> {
    use schema::workflow_executions::dsl::*;

    let query = insert_into(workflow_executions)
        .values(form)
        .returning(execution_id)
        .get_result(conn)
        .await?;

    Ok(WorkflowExecutionCreateOutput {
        execution_id: query,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionListInput {
    pub workflow_id: Uuid,
    pub limit: Option<i64>,
    pub offset: Option<i64>,

    pub sort_by: WorkflowsSortBy,
    pub sort_order: QueryOrderBy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "forms do nothing unless you use them"]
pub enum WorkflowsSortBy {
    StartedAt,
    EndedAt,
    TimeSpent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::workflow_executions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionListOutput {
    pub workflow_id: Uuid,
}

/// Returns a set of workflow executions IDs.
///
/// # Tables
///
/// - workflow_executions
pub async fn list_workflow_executions(
    conn: &mut AsyncPgConnection,
    form: WorkflowExecutionListInput,
) -> DatabaseResult<Vec<WorkflowExecutionListOutput>> {
    use schema::workflow_executions::dsl::*;

    let query = match (form.sort_by, form.sort_order) {
        (WorkflowsSortBy::StartedAt, QueryOrderBy::Ascending) => {
            workflow_executions.order_by(started_at.asc()).into_boxed()
        }
        (WorkflowsSortBy::StartedAt, QueryOrderBy::Descending) => {
            workflow_executions.order_by(started_at.desc()).into_boxed()
        }
        (WorkflowsSortBy::EndedAt, QueryOrderBy::Ascending) => {
            workflow_executions.order_by(ended_at.desc()).into_boxed()
        }
        (WorkflowsSortBy::EndedAt, QueryOrderBy::Descending) => {
            workflow_executions.order_by(ended_at.desc()).into_boxed()
        }
        (WorkflowsSortBy::TimeSpent, QueryOrderBy::Ascending) => workflow_executions
            .order_by(age(ended_at, started_at).asc())
            .into_boxed(),
        (WorkflowsSortBy::TimeSpent, QueryOrderBy::Descending) => workflow_executions
            .order_by(age(ended_at, started_at).desc())
            .into_boxed(),
    };

    let query = query
        .filter(workflow_id.eq(form.workflow_id).and(deleted_at.is_null()))
        .select(WorkflowExecutionListOutput::as_select())
        .offset(form.offset.unwrap_or_default())
        .limit(form.limit.unwrap_or(20))
        .get_results(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionViewInput {
    pub execution_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::workflow_executions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionViewOutput {
    pub workflow_id: Uuid,
    pub execution_id: Uuid,
    pub output_graph: Value,
    pub rt_metadata: Value,

    pub started_at: OffsetDateTime,
    pub ended_at: OffsetDateTime,
}

/// Returns the details of the specified workflow execution.
///
/// # Tables
///
/// - workflow_executions
pub async fn view_workflow_execution(
    conn: &mut AsyncPgConnection,
    form: WorkflowExecutionViewInput,
) -> DatabaseResult<WorkflowExecutionViewOutput> {
    use schema::workflow_executions::dsl::*;

    let query = workflow_executions
        .filter(execution_id.eq(form.execution_id).and(deleted_at.is_null()))
        .select(WorkflowExecutionViewOutput::as_select())
        .limit(1)
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowExecutionDeleteInput {
    pub execution_id: Uuid,
}

/// Flags the specified workflow execution as deleted.
///
/// # Tables
///
/// - workflow_executions
pub async fn delete_workflow_execution(
    conn: &mut AsyncPgConnection,
    form: WorkflowExecutionDeleteInput,
) -> DatabaseResult<()> {
    use schema::workflow_executions::dsl::*;

    let filter_cond =
        workflow_executions.filter(execution_id.eq(form.execution_id).and(deleted_at.is_null()));

    let _query = update(filter_cond)
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
