use diesel::dsl::{insert_into, now, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{Database, DatabaseResult};

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountCreateForm<'a> {
    pub display_name: &'a str,
    pub email_address: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountViewForm {
    pub display_name: String,
    pub email_address: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountUpdateForm {
    pub display_name: Option<String>,
    pub email_address: Option<String>,
    pub password_hash: Option<String>,
}

/// Queries for the `accounts` table.
pub trait AccountsExt {
    /// - Creates a new account.
    /// - Returns the account's id.
    ///
    /// # Tables
    ///
    /// - accounts
    async fn create_account(
        &self,
        conn: &mut AsyncPgConnection,
        form: AccountCreateForm<'_>,
    ) -> DatabaseResult<Uuid>;

    /// - Return an account.
    ///
    /// # Tables
    ///
    /// - accounts
    async fn view_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<Option<AccountViewForm>>;

    /// - Updates the account.
    ///
    /// # Tables
    ///
    /// - accounts
    async fn update_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountUpdateForm,
    ) -> DatabaseResult<()>;

    /// - Flags the account as deleted.
    ///
    /// # Tables
    ///
    /// - accounts
    async fn delete_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;
}

impl AccountsExt for Database {
    async fn create_account(
        &self,
        conn: &mut AsyncPgConnection,
        form: AccountCreateForm<'_>,
    ) -> DatabaseResult<Uuid> {
        use crate::schema::accounts::dsl::*;

        let query = insert_into(accounts)
            .values(form)
            .returning(id)
            .get_result(conn)
            .await?;

        Ok(query)
    }

    async fn view_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<Option<AccountViewForm>> {
        use crate::schema::accounts::dsl::*;

        let query = accounts
            .filter(id.eq(account_id_recv).and(deleted_at.is_not_null()))
            .select(AccountViewForm::as_select())
            .get_result(conn)
            .await
            .optional()?;

        Ok(query)
    }

    async fn update_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountUpdateForm,
    ) -> DatabaseResult<()> {
        use crate::schema::accounts::dsl::*;

        let _query = update(accounts.filter(id.eq(account_id_recv).and(deleted_at.is_not_null())))
            .set(form)
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn delete_account(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()> {
        use crate::schema::accounts::dsl::*;

        let filter_cond = id.eq(account_id_recv).and(deleted_at.is_not_null());

        let _query = update(accounts.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
