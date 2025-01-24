//! Data layer for workflow management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::workspaces::WorkspaceViewOutput;
use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateInputForm<'a> {
    pub workspace_id: Uuid,
    pub display_name: Option<&'a str>,
    pub metadata: Option<Value>,
    pub input_graph: Option<Value>,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateOutputForm {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

/// Creates a new workflow in the database.
///
/// # Tables
///
/// - workflows
pub async fn create_workflow(
    conn: &mut AsyncPgConnection,
    form: &WorkflowCreateInputForm<'_>,
) -> DatabaseResult<WorkflowCreateOutputForm> {
    use schema::workflows::dsl::*;

    let query = diesel::insert_into(workflows)
        .values(form)
        .returning((id, created_at, updated_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowUpdateInputForm<'a> {
    pub display_name: Option<&'a str>,
}

pub async fn update_workflow(
    conn: &mut AsyncPgConnection,
    form: WorkflowUpdateInputForm<'_>,
) -> DatabaseResult<()> {
    Ok(())
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowViewOutput {
    pub id: Uuid,
    pub display_name: String,
    pub metadata: Value,

    pub input_graph: Value,
    pub rt_metadata: Value,

    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

/// TODO.
///
/// # Tables
///
/// - workflows
pub async fn view_workflows_by_id(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
) -> DatabaseResult<WorkflowViewOutput> {
    use schema::workflows::dsl::*;

    let filter_cond = id.eq(form_workflow_id).and(deleted_at.is_null());
    let workflow = workflows
        .filter(filter_cond)
        .select(WorkflowViewOutput::as_select())
        .limit(1)
        .get_result(conn)
        .await?;

    Ok(workflow)
}

/// Marks a workflow as deleted.
///
/// # Tables
///
/// - workflows
pub async fn delete_workflow(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workflows::dsl::*;

    let filter_cond = id.eq(form_workflow_id).and(deleted_at.is_null());
    update(workflows.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
