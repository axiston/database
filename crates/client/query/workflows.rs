//! Data layer for workflow management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateInput<'a> {
    pub workspace_id: Uuid,
    pub display_name: Option<&'a str>,
    pub metadata: Option<Value>,
    pub input_graph: Option<Value>,
}

#[derive(Debug, Clone, Queryable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowCreateOutput {
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

#[derive(Debug, Clone, AsChangeset)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkflowUpdateInput<'a> {
    pub display_name: Option<&'a str>,
}

pub async fn update_workflow(
    conn: &mut AsyncPgConnection,
    form: WorkflowUpdateInput<'_>,
) -> DatabaseResult<()> {
    todo!()
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
