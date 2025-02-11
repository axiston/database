//! Data layer for account management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountCreateInput<'a> {
    pub display_name: &'a str,
    pub email_address: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountCreateOutput {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Creates the new account and returns its ID.
///
/// # Tables
///
/// - accounts
pub async fn create_account(
    conn: &mut AsyncPgConnection,
    form: &AccountCreateInput<'_>,
) -> DatabaseResult<AccountCreateOutput> {
    use schema::accounts::dsl::*;

    let query = insert_into(accounts)
        .values(form)
        .returning((id, created_at, updated_at, deleted_at))
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountViewOutput {
    pub display_name: String,
    pub email_address: String,
    pub password_hash: String,
    pub is_activated: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Returns the account data by its ID.
///
/// # Tables
///
/// - accounts
pub async fn view_account(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
) -> DatabaseResult<AccountViewOutput> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(form_account_id).and(deleted_at.is_null());
    let query = accounts
        .filter(filter_cond)
        .select(AccountViewOutput::as_select())
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountUpdateInput<'a> {
    pub display_name: Option<&'a str>,
    pub email_address: Option<&'a str>,
    pub password_hash: Option<&'a str>,
    pub is_activated: Option<bool>,
}

/// Updates the account with provided data.
///
/// # Tables
///
/// - accounts
pub async fn update_account(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
    form: AccountUpdateInput<'_>,
) -> DatabaseResult<()> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(form_account_id).and(deleted_at.is_null());
    let _query = update(accounts.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Flags the account as deleted.
///
/// # Tables
///
/// - accounts
pub async fn delete_account(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
) -> DatabaseResult<()> {
    use schema::accounts::dsl::*;

    let filter_cond = id.eq(form_account_id).and(deleted_at.is_null());
    let _query = update(accounts.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
