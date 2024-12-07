use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{Database, DatabaseResult};

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::account_permissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountPermissionsForm {
    pub nocheck_read: bool,
    pub nocheck_write: bool,
}

/// Queries for the `account_permissions` table.
pub trait AccountPermissionsExt {
    /// Returns the account's permissions.
    ///
    /// # Tables
    ///
    /// - account_permissions
    async fn find_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<AccountPermissionsForm>;

    /// Automatically creates or updates permissions.
    ///
    /// # Tables
    ///
    /// - account_permissions
    async fn update_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountPermissionsForm,
    ) -> DatabaseResult<()>;

    /// Deletes the account's permissions.
    ///
    /// # Tables
    ///
    /// - account_permissions
    async fn delete_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<AccountPermissionsForm>;
}

impl AccountPermissionsExt for Database {
    async fn find_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<AccountPermissionsForm> {
        use crate::schema::account_permissions::dsl::*;

        let query = account_permissions
            .filter(account_id.eq(account_id_recv))
            .select(AccountPermissionsForm::as_select())
            .get_result(conn)
            .await?;

        Ok(query)
    }

    async fn update_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountPermissionsForm,
    ) -> DatabaseResult<()> {
        use crate::schema::account_permissions::dsl::*;

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

    async fn delete_permissions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<AccountPermissionsForm> {
        use crate::schema::account_permissions::dsl::*;

        let query = delete(account_permissions.filter(account_id.eq(account_id_recv)))
            .returning(AccountPermissionsForm::as_returning())
            .get_result(conn)
            .await?;

        Ok(query)
    }
}
