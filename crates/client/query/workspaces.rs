//! Data layer for workspace management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceCreateInputForm<'a> {
    pub display_name: &'a str,
    pub metadata_props: &'a serde_json::Value,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceCreateOutputForm {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceViewOutputForm {
    pub id: Uuid,
    pub display_name: String,
    pub metadata_props: serde_json::Value,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::workspaces)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceUpdateInputForm<'a> {
    pub display_name: Option<&'a str>,
    pub metadata_props: Option<&'a serde_json::Value>,
}

/// Creates a new workspace and returns its details.
///
/// # Tables
///
/// - workspaces
pub async fn create_workspace(
    conn: &mut AsyncPgConnection,
    workspace_form: &WorkspaceCreateInputForm<'_>,
) -> DatabaseResult<WorkspaceCreateOutputForm> {
    use schema::workspaces::dsl::*;

    let query = insert_into(workspaces)
        .values(workspace_form)
        .returning((id, created_at, updated_at, deleted_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Retrieves a workspace by its unique ID.
///
/// # Tables
///
/// - workspaces
pub async fn view_workspace(
    conn: &mut AsyncPgConnection,
    workspace_id_val: Uuid,
) -> DatabaseResult<WorkspaceViewOutputForm> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(workspace_id_val).and(deleted_at.is_null());
    let query = workspaces
        .filter(filter_cond)
        .select(WorkspaceViewOutputForm::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates a workspace's details.
///
/// # Tables
///
/// - workspaces
pub async fn update_workspace(
    conn: &mut AsyncPgConnection,
    workspace_id_val: Uuid,
    form: WorkspaceUpdateInputForm<'_>,
) -> DatabaseResult<()> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(workspace_id_val).and(deleted_at.is_null());
    let _query = update(workspaces.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Marks a workspace as deleted.
///
/// # Tables
///
/// - workspaces
pub async fn delete_workspace(
    conn: &mut AsyncPgConnection,
    workspace_id_val: Uuid,
) -> DatabaseResult<()> {
    use schema::workspaces::dsl::*;

    let filter_cond = id.eq(workspace_id_val).and(deleted_at.is_null());
    let _query = update(workspaces.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
