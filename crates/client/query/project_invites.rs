use axiston_db_schema::enumerations::InviteStatusForm;
use axiston_db_schema::schema;
use diesel::dsl::{insert_into, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct CreateInviteInput {
    pub account_id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct ViewInviteOutput {
    pub account_id: Uuid,
    pub project_id: Uuid,
    pub status: InviteStatusForm,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::project_invites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct UpdateInviteInput {
    pub account_id: Uuid,
    pub project_id: Uuid,
    pub updated_by: Uuid,
    pub status: InviteStatusForm,
}

/// - Inserts the new invitation.
///
/// # Tables
///
/// - project_invites
pub async fn create_invite(
    conn: &mut AsyncPgConnection,
    invite: CreateInviteInput,
) -> DatabaseResult<()> {
    use schema::project_invites::dsl::*;

    let _query = insert_into(project_invites)
        .values((
            account_id.eq(invite.account_id),
            project_id.eq(invite.project_id),
            created_by.eq(invite.created_by),
        ))
        .execute(conn)
        .await?;

    Ok(())
}

/// - Updates an invitation status.
///
/// # Tables
///
/// - project_invites
pub async fn update_pending_invite_status(
    conn: &mut AsyncPgConnection,
    invite: UpdateInviteInput,
) -> DatabaseResult<()> {
    use schema::project_invites::dsl::*;

    let filter_cond = project_id
        .eq(invite.project_id)
        .and(account_id.eq(invite.account_id))
        .and(status.eq(InviteStatusForm::Pending));

    let _query = update(project_invites.filter(filter_cond))
        .set((updated_by.eq(invite.updated_by), status.eq(invite.status)))
        .execute(conn)
        .await?;

    Ok(())
}

/// - Returns a single pending invite.
///
/// # Tables
///
/// - project_invites
pub async fn view_pending_invite(
    conn: &mut AsyncPgConnection,
    project_id_recv: Uuid,
    account_id_recv: Uuid,
) -> DatabaseResult<Option<ViewInviteOutput>> {
    use schema::project_invites::dsl::*;

    let filter_cond = project_id
        .eq(project_id_recv)
        .and(account_id.eq(account_id_recv))
        .and(status.eq(InviteStatusForm::Pending));

    let query = project_invites
        .filter(filter_cond)
        .select(ViewInviteOutput::as_select())
        .get_result(conn)
        .await
        .optional()?;

    Ok(query)
}

/// - Lists all active invites of the project.
///
/// # Tables
///
/// - project_invites
pub async fn view_pending_project_invites(
    conn: &mut AsyncPgConnection,
    project_id_recv: Uuid,
) -> DatabaseResult<Vec<ViewInviteOutput>> {
    use schema::project_invites::dsl::*;

    let filter_cond = project_id
        .eq(project_id_recv)
        .and(status.eq(InviteStatusForm::Pending));

    let query = project_invites
        .filter(filter_cond)
        .select(ViewInviteOutput::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}

/// - Lists all active invites of the project.
///
/// # Tables
///
/// - project_invites
pub async fn view_pending_account_invites(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<Vec<ViewInviteOutput>> {
    use schema::project_invites::dsl::*;

    let filter_cond = account_id
        .eq(account_id_recv)
        .and(status.eq(InviteStatusForm::Pending));

    let query = project_invites
        .filter(filter_cond)
        .select(ViewInviteOutput::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}
