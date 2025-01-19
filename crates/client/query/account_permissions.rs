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
    pub nocheck_read: bool,
    pub nocheck_write: bool,
}

/// Automatically creates or updates permissions.
///
/// # Tables
///
/// - account_permissions
pub async fn update_permissions(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
    form: AccountPermissionsForm,
) -> DatabaseResult<()> {
    use schema::account_permissions::dsl::*;

    let _query = insert_into(account_permissions)
        .values((
            account_id.eq(account_id_recv),
            nocheck_read.eq(form.nocheck_read),
            nocheck_write.eq(form.nocheck_write),
        ))
        .on_conflict(account_id)
        .do_update()
        .set((
            nocheck_read.eq(form.nocheck_read),
            nocheck_write.eq(form.nocheck_write),
        ))
        .execute(conn)
        .await?;

    Ok(())
}

/// Returns the account's permissions.
///
/// # Tables
///
/// - account_permissions
pub async fn find_permissions(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<AccountPermissionsForm> {
    use schema::account_permissions::dsl::*;

    let filter_cond = account_id.eq(account_id_recv).and(deleted_at.is_null());
    let query = account_permissions
        .filter(filter_cond)
        .select(AccountPermissionsForm::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Deletes the account permissions by its id.
///
/// # Tables
///
/// - account_permissions
pub async fn delete_permissions(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<()> {
    use schema::account_permissions::dsl::*;

    let filter_cond = account_id.eq(account_id_recv).and(deleted_at.is_null());
    let _query = update(account_permissions.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
