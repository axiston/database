//! Data layer for account permissions management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::account_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountPermissionsForm {
    pub read_accounts: bool,
    pub write_accounts: bool,

    pub read_workspaces: bool,
    pub write_workspaces: bool,

    pub read_workflows: bool,
    pub write_workflows: bool,
}

/// Automatically creates or updates permissions.
///
/// # Tables
///
/// - account_permissions
pub async fn update_permissions(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
    form: AccountPermissionsForm,
) -> DatabaseResult<()> {
    use schema::account_permissions::dsl::*;

    let _query = insert_into(account_permissions)
        .values((
            account_id.eq(form_account_id),
            read_accounts.eq(form.read_accounts),
            write_accounts.eq(form.write_accounts),
            read_workspaces.eq(form.read_workspaces),
            write_workspaces.eq(form.write_workspaces),
            read_workflows.eq(form.read_workflows),
            write_workflows.eq(form.write_workflows),
        ))
        .on_conflict(account_id)
        .do_update()
        .set((
            read_accounts.eq(form.read_accounts),
            read_accounts.eq(form.write_accounts),
            read_workspaces.eq(form.read_workspaces),
            write_workspaces.eq(form.write_workspaces),
            read_workflows.eq(form.read_workflows),
            write_workflows.eq(form.write_workflows),
        ))
        .execute(conn)
        .await?;

    Ok(())
}

/// Returns the account permissions by account ID.
///
/// # Tables
///
/// - account_permissions
pub async fn find_permissions(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
) -> DatabaseResult<AccountPermissionsForm> {
    use schema::account_permissions::dsl::*;

    let filter_cond = account_id.eq(form_account_id);
    let query = account_permissions
        .filter(filter_cond)
        .select(AccountPermissionsForm::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}
