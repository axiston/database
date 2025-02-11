//! Data layer for workspace management.

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
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceCreateInput<'a> {
    pub display_name: &'a str,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceCreateOutput {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Creates a new workspace and returns its details.
///
/// # Tables
///
/// - workspaces
pub async fn create_workspace(
    conn: &mut AsyncPgConnection,
    workspace_form: &WorkspaceCreateInput<'_>,
) -> DatabaseResult<WorkspaceCreateOutput> {
    use schema::workspaces::dsl::*;

    let query = insert_into(workspaces)
        .values(workspace_form)
        .returning((id, created_at, updated_at, deleted_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceViewOutput {
    pub id: Uuid,
    pub display_name: String,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Retrieves a workspace by its unique ID.
///
/// # Tables
///
/// - workspaces
pub async fn view_workspace(
    conn: &mut AsyncPgConnection,
    form_workspace_id: Uuid,
) -> DatabaseResult<WorkspaceViewOutput> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(form_workspace_id).and(deleted_at.is_null());
    let query = workspaces
        .filter(filter_cond)
        .select(WorkspaceViewOutput::as_select())
        .limit(1)
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceUpdateInput<'a> {
    pub display_name: Option<&'a str>,
    pub metadata: Value,
}

/// Updates a workspace's details.
///
/// # Tables
///
/// - workspaces
pub async fn update_workspace(
    conn: &mut AsyncPgConnection,
    form_workspace_id: Uuid,
    form: WorkspaceUpdateInput<'_>,
) -> DatabaseResult<()> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(form_workspace_id).and(deleted_at.is_null());
    let _query = update(workspaces.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Flags the specified workspace as deleted.
///
/// # Tables
///
/// - workspaces
pub async fn delete_workspace(
    conn: &mut AsyncPgConnection,
    form_workspace_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(form_workspace_id).and(deleted_at.is_null());
    let _query = update(workspaces.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
