//! Data layer for workspace invitations.

use axiston_db_schema::enumerations::InviteStatus;
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
#[diesel(table_name = schema::workspace_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceInviteCreateInput {
    pub workspace_id: Uuid,
    pub account_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Queryable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceInviteCreateOutput {
    pub workspace_id: Uuid,
    pub invite_id: Uuid,
    pub status: InviteStatus,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceInviteViewOutput {
    pub workspace_id: Uuid,
    pub invite_id: Uuid,
    pub invite_status: InviteStatus,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceInviteUpdateInput {
    pub invite_status: InviteStatus,
    pub updated_by: Uuid,
}

/// Creates a new workspace invitation.
///
/// # Tables
///
/// - workspace_invites
pub async fn create_workspace_invite(
    conn: &mut AsyncPgConnection,
    invite_form: &WorkspaceInviteCreateInput,
) -> DatabaseResult<WorkspaceInviteCreateOutput> {
    use schema::workspace_invites::dsl::*;

    let query = insert_into(workspace_invites)
        .values(invite_form)
        .returning((
            workspace_id,
            invite_id,
            invite_status,
            created_at,
            updated_at,
        ))
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates the status of a workspace invitation.
///
/// # Tables
///
/// - workspace_invites
pub async fn update_workspace_invite(
    conn: &mut AsyncPgConnection,
    workspace_id_val: Uuid,
    invite_id_val: Uuid,
    form: WorkspaceInviteUpdateInput,
) -> DatabaseResult<()> {
    use schema::workspace_invites::dsl::*;

    let filter_cond = workspace_id
        .eq(workspace_id_val)
        .and(invite_id.eq(invite_id_val));
    let _query = update(workspace_invites.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Retrieves an invitation by workspace and invite ID.
///
/// # Tables
///
/// - workspace_invites
pub async fn view_workspace_invite(
    conn: &mut AsyncPgConnection,
    form_workspace_id: Uuid,
    invite_id_val: Uuid,
) -> DatabaseResult<WorkspaceInviteViewOutput> {
    use schema::workspace_invites::dsl::*;

    let filter_cond = workspace_id
        .eq(form_workspace_id)
        .and(invite_id.eq(invite_id_val));
    let query = workspace_invites
        .filter(filter_cond)
        .select(WorkspaceInviteViewOutput::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}
