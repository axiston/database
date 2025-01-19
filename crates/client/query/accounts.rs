//! Data layer for account management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountCreateInputForm<'a> {
    pub display_name: &'a str,
    pub email_address: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountCreateOutputForm {
    pub id: Uuid,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountViewOutputForm {
    pub display_name: String,
    pub email_address: String,
    pub password_hash: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountUpdateInputForm<'a> {
    pub display_name: Option<&'a str>,
    pub email_address: Option<&'a str>,
    pub password_hash: Option<&'a str>,
}

/// Creates the new account and returns its id.
///
/// # Tables
///
/// - account
pub async fn create_account(
    conn: &mut AsyncPgConnection,
    account_form: &AccountCreateInputForm<'_>,
) -> DatabaseResult<AccountCreateOutputForm> {
    use schema::accounts::dsl::*;

    let query = insert_into(accounts)
        .values(account_form)
        .returning((id, created_at, updated_at, deleted_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Returns the account id by its email.
///
/// # Tables
///
/// - account
pub async fn find_account_id_by_email(
    conn: &mut AsyncPgConnection,
    account_id_email: &str,
) -> DatabaseResult<Uuid> {
    use schema::accounts::dsl::*;

    let filter_cond = email_address.eq(account_id_email).and(deleted_at.is_null());
    let query = accounts
        .filter(filter_cond)
        .select(id)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Returns the account data by its id.
///
/// # Tables
///
/// - account
pub async fn view_account(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<AccountViewOutputForm> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(account_id_recv).and(deleted_at.is_null());
    let query = accounts
        .filter(filter_cond)
        .select(AccountViewOutputForm::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates the account with provided update date.
///
/// # Tables
///
/// - account
pub async fn update_account(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
    form: AccountUpdateInputForm<'_>,
) -> DatabaseResult<()> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(account_id_recv).and(deleted_at.is_null());
    let _query = update(accounts.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Finds the account by its id and flags it as deleted.
///
/// # Tables
///
/// - account
pub async fn delete_account(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
) -> DatabaseResult<()> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(account_id_recv).and(deleted_at.is_not_null());
    let _query = update(accounts.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
