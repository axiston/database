//! Data layer for managing workspace schedules.

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
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleCreateInput {
    pub workspace_id: Uuid,
    pub metadata: Value,
}

#[derive(Debug, Clone, Queryable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleCreateOutput {
    pub id: Uuid,
}

/// Creates a new workspace schedule.
///
/// # Tables
///
///  - workspace_schedules
pub async fn create_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_form: &WorkspaceScheduleCreateInput,
) -> DatabaseResult<WorkspaceScheduleCreateOutput> {
    use schema::workspace_schedules::dsl::*;

    let query = insert_into(workspace_schedules)
        .values(schedule_form)
        .returning((id,))
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Queryable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleViewOutput {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub update_interval: i32,
    pub metadata: Value,

    #[cfg_attr(feature = "serde", serde(with = "crate::serde::iso8601"))]
    pub created_at: PrimitiveDateTime,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::iso8601"))]
    pub updated_at: PrimitiveDateTime,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::iso8601::option"))]
    pub deleted_at: Option<PrimitiveDateTime>,
}

/// Retrieves a workspace schedule by its ID.
///
/// # Tables
///
///  - workspace_schedules
pub async fn view_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_id: Uuid,
) -> DatabaseResult<WorkspaceScheduleViewOutput> {
    use schema::workspace_schedules::dsl::*;

    let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
    let query = workspace_schedules
        .filter(filter_cond)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// TODO.
///
/// # Tables
///
/// - workflow_schedules
pub async fn view_workflows_by_interval(
    conn: &mut AsyncPgConnection,
) -> DatabaseResult<Vec<WorkspaceScheduleViewOutput>> {
    // TODO: Loads all workflows in the asc interval order.
    todo!()
}

#[derive(Debug, Clone, AsChangeset)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleUpdateInput {
    pub metadata: Option<Value>,
}

/// Updates a workspace schedule.
///
/// # Tables
///
///  - workspace_schedules
pub async fn update_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_id: Uuid,
    form: WorkspaceScheduleUpdateInput,
) -> DatabaseResult<()> {
    use schema::workspace_schedules::dsl::*;

    let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
    update(workspace_schedules.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Flags the workspace schedule as deleted.
///
/// # Tables
///
///  - workspace_schedules
pub async fn delete_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workspace_schedules::dsl::*;

    let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
    update(workspace_schedules.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
