//! Data layer for workspace member management.

use axiston_db_schema::enumerations::ProjectRole;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::workspace_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceMemberCreateInput {
    pub workspace_id: Uuid,
    pub account_id: Uuid,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
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

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, AsChangeset)]
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
    form_workspace_id: Uuid,
    form_account_id: Uuid,
) -> DatabaseResult<WorkspaceMemberOutput> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(form_workspace_id)
        .and(account_id.eq(form_account_id));

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
    form_workspace_id: Uuid,
    form_account_id: Uuid,
    form: WorkspaceMemberUpdateInput,
) -> DatabaseResult<()> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(form_workspace_id)
        .and(account_id.eq(form_account_id));

    let _query = update(workspace_members.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Deletes a member from a workspace.
///
/// # Tables
///
/// - workspace_members
pub async fn remove_workspace_member(
    conn: &mut AsyncPgConnection,
    form_workspace_id: Uuid,
    form_account_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workspace_members::dsl::*;

    let filter_cond = workspace_id
        .eq(form_workspace_id)
        .and(account_id.eq(form_account_id));

    let _query = delete(workspace_members.filter(filter_cond))
        .execute(conn)
        .await?;

    Ok(())
}
