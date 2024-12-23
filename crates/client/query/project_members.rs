use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct CreateMemberInput {
    pub account_id: Uuid,
    pub project_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct ViewMembersOutput {
    pub account_id: Uuid,
    pub project_id: Uuid,

    pub show_order: i32,
    pub is_pinned: bool,
    pub is_hidden: bool,
}

/// - Inserts an active member to the project.
///
/// # Tables
///
/// - project_members
pub async fn create_member(
    conn: &mut AsyncPgConnection,
    member: CreateMemberInput,
) -> DatabaseResult<()> {
    use schema::project_members::dsl::*;

    let _query = insert_into(project_members)
        .values((
            account_id.eq(member.account_id),
            project_id.eq(member.project_id),
            created_by.eq(member.created_by),
        ))
        .execute(conn)
        .await?;

    Ok(())
}

/// - Kicks an active member from the project.
///
/// # Tables
///
/// - project_members
pub async fn delete_member(
    conn: &mut AsyncPgConnection,
    project_id_recv: Uuid,
    account_id_send: Uuid,
    account_id_recv: Uuid,
) -> DatabaseResult<()> {
    use schema::project_members::dsl::*;

    let filter_cond = account_id
        .eq(account_id_recv)
        .and(project_id.eq(project_id_recv));

    let _query = update(project_members.filter(filter_cond))
        .set((updated_at.eq(now), updated_by.eq(account_id_send)))
        .execute(conn)
        .await?;

    Ok(())
}

/// - Lists all projects.
///
/// # Tables
///
/// - projects
/// - project_members
pub async fn view_member_projects(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<Vec<ViewMembersOutput>> {
    use schema::project_members::dsl::*;

    let filter_cond = account_id.eq(account_id_recv);
    let query = project_members
        .filter(filter_cond)
        .select(ViewMembersOutput::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}

/// - Lists all active members of the project.
///
/// # Tables
///
/// - project_members
pub async fn view_project_members(
    conn: &mut AsyncPgConnection,
    project_id_recv: Uuid,
) -> DatabaseResult<Vec<ViewMembersOutput>> {
    use schema::project_members::dsl::*;

    let filter_cond = project_id.eq(project_id_recv);
    let query = project_members
        .filter(filter_cond)
        .select(ViewMembersOutput::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}
