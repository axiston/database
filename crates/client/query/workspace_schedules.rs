//! Data layer for managing workspace schedules.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleCreateInputForm<'a> {
    pub workspace_id: Uuid,
    pub metadata: &'a Value,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleOutputForm {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub update_interval: i32,
    pub metadata: Value,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleUpdateInputForm<'a> {
    pub metadata: Option<&'a Value>,
}

/// Creates a new workspace schedule.
///
/// # Tables
///
///  - workspace_schedules
pub async fn create_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_form: &WorkspaceScheduleCreateInputForm<'_>,
) -> DatabaseResult<WorkspaceScheduleOutputForm> {
    use schema::workspace_schedules::dsl::*;

    let query = insert_into(workspace_schedules)
        .values(schedule_form)
        .returning((
            id,
            workspace_id,
            update_interval,
            metadata,
            created_at,
            updated_at,
            deleted_at,
        ))
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Retrieves a workspace schedule by its ID.
///
/// # Tables
///
///  - workspace_schedules
pub async fn view_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_id: Uuid,
) -> DatabaseResult<WorkspaceScheduleOutputForm> {
    use schema::workspace_schedules::dsl::*;

    let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
    let query = workspace_schedules
        .filter(filter_cond)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates a workspace schedule.
///
/// # Tables
///
///  - workspace_schedules
pub async fn update_workspace_schedule(
    conn: &mut AsyncPgConnection,
    schedule_id: Uuid,
    form: WorkspaceScheduleUpdateInputForm<'_>,
) -> DatabaseResult<()> {
    use schema::workspace_schedules::dsl::*;

    let filter_cond = id.eq(schedule_id).and(deleted_at.is_null());
    update(workspace_schedules.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Marks a workspace schedule as deleted.
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
