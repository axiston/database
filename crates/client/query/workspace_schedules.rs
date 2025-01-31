//! Data layer for managing workspace schedules.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Timestamptz};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
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

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceScheduleViewOutput {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub update_interval: i32,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
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

/// Retrieves a batch of workflow schedules.
///
/// The batch contains the first n rows in the table sorted
/// by the last modification date + update interval.
///
/// Also updates the modification timestamp.
///
/// # Tables
///
/// - workspace_schedules
pub async fn view_workflows_by_interval(
    conn: &mut AsyncPgConnection,
    max_batch_size: i64,
    max_timeout: OffsetDateTime,
) -> DatabaseResult<Vec<WorkspaceScheduleViewOutput>> {
    use schema::workflow_schedules::dsl as wfs_dsl;
    use schema::workflows::dsl as wf_dsl;
    use schema::workspace_schedules::dsl as wss_dsl;

    let filter_future_schedules = sql::<Bool>(
        "(workflows.updated_at + workspace_schedules.update_interval * interval '1 second') <= $1",
    )
    .bind::<Timestamptz, _>(max_timeout);

    let order_by_interval = sql::<Timestamptz>(
        "workflows.updated_at + workspace_schedules.update_interval * interval '1 second' ASC",
    );

    let queries = wf_dsl::workflows
        .inner_join(wfs_dsl::workflow_schedules.on(wfs_dsl::workflow_id.eq(wf_dsl::id)))
        .inner_join(wss_dsl::workspace_schedules.on(wss_dsl::id.eq(wfs_dsl::schedule_id)))
        .filter(wf_dsl::deleted_at.is_null())
        .filter(filter_future_schedules)
        .order_by(order_by_interval)
        .select(WorkspaceScheduleViewOutput::as_select())
        .limit(max_batch_size)
        .for_update()
        .get_results(conn)
        .await?;

    let schedule_ids: Vec<_> = queries.iter().map(|ws| ws.id).collect();
    let _query = update(wss_dsl::workspace_schedules)
        .filter(wss_dsl::id.eq_any(schedule_ids))
        .set(wss_dsl::updated_at.eq(now))
        .execute(conn)
        .await?;

    Ok(queries)
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
