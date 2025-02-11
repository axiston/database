//! Data layer for workflow management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateInput<'a> {
    pub workspace_id: Uuid,
    pub display_name: Option<&'a str>,
    pub metadata: Option<Value>,
    pub input_graph: Option<Value>,
    pub rt_metadata: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateOutput {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Creates a new workflow in the database.
///
/// # Tables
///
/// - workflows
pub async fn create_workflow(
    conn: &mut AsyncPgConnection,
    form: &WorkflowCreateInput<'_>,
) -> DatabaseResult<WorkflowCreateOutput> {
    use schema::workflows::dsl::*;

    let query = diesel::insert_into(workflows)
        .values(form)
        .returning((id, created_at, updated_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowUpdateInput<'a> {
    pub display_name: Option<&'a str>,
    pub metadata: Option<Value>,
    pub input_graph: Option<Value>,
    pub rt_metadata: Option<Value>,
}

/// Updates the workflow with provided data.
///
/// # Tables
///
/// - workflows
pub async fn update_workflow(
    conn: &mut AsyncPgConnection,
    form_workflow_id: Uuid,
    form: WorkflowUpdateInput<'_>,
) -> DatabaseResult<()> {
    use schema::workflows::dsl::*;

    let filter_cond = id.eq(form_workflow_id).and(deleted_at.is_null());
    let _query = update(workflows.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowViewOutput {
    pub id: Uuid,
    pub display_name: String,
    pub metadata: Value,

    pub input_graph: Value,
    pub rt_metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Returns the workflow data by its ID.
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

/// Flags a specified workflow as deleted.
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
