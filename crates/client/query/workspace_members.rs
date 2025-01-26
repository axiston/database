//! Data layer for workspace member management.

use axiston_db_schema::enumerations::ProjectRole;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceMemberCreateInput {
    pub workspace_id: Uuid,
    pub account_id: Uuid,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceMemberOutput {
    pub workspace_id: Uuid,
    pub account_id: Uuid,
    pub show_order: i32,
    pub is_pinned: bool,
    pub is_hidden: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Default, Clone, AsChangeset)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceMemberUpdateInput {
    pub show_order: Option<i32>,
    pub is_pinned: Option<bool>,
    pub is_hidden: Option<bool>,
    pub account_role: Option<ProjectRole>,
    pub updated_by: Option<Uuid>,
}

/// Adds a new member to a workspace.
///
/// # Tables
///
/// - workspace_members
pub async fn create_workspace_member(
    conn: &mut AsyncPgConnection,
    member_form: &WorkspaceMemberCreateInput,
) -> DatabaseResult<()> {
    use schema::workspace_members::dsl::*;

    let _query = insert_into(workspace_members)
        .values(member_form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Retrieves a member of a workspace.
///
/// # Tables
///
/// - workspace_members
pub async fn get_workspace_member(
    conn: &mut AsyncPgConnection,
    workspace_id_recv: Uuid,
    account_id_recv: Uuid,
) -> DatabaseResult<WorkspaceMemberOutput> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(workspace_id_recv)
        .and(account_id.eq(account_id_recv));

    let query = workspace_members
        .filter(filter_cond)
        .select(WorkspaceMemberOutput::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates a member's details in a workspace.
///
/// # Tables
///
/// - workspace_members
pub async fn update_workspace_member(
    conn: &mut AsyncPgConnection,
    workspace_id_recv: Uuid,
    account_id_recv: Uuid,
    form: WorkspaceMemberUpdateInput,
) -> DatabaseResult<()> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(workspace_id_recv)
        .and(account_id.eq(account_id_recv));

    let _query = update(workspace_members.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Removes a member from a workspace.
///
/// # Tables
///
/// - workspace_members
pub async fn remove_workspace_member(
    conn: &mut AsyncPgConnection,
    workspace_id_recv: Uuid,
    account_id_recv: Uuid,
) -> DatabaseResult<()> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(workspace_id_recv)
        .and(account_id.eq(account_id_recv));

    let _query = delete(workspace_members.filter(filter_cond))
        .execute(conn)
        .await?;

    Ok(())
}
