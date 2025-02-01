//! Data layer for account tokens management.

use axiston_db_schema::enumerations::TokenAction;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountTokenCreateInput {
    pub account_id: Uuid,
    pub action_type: TokenAction,
    pub token_data: serde_json::Value,
    pub ip_address: IpNet,
    pub user_agent: String,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountTokenCreateOutput {
    pub action_token: Uuid,
}

/// Creates and returns the new action token.
///
/// # Tables
///
/// - account_emails
pub async fn create_action_token(
    conn: &mut AsyncPgConnection,
    form: AccountTokenCreateInput,
) -> DatabaseResult<AccountTokenCreateOutput> {
    use schema::account_tokens::dsl::*;

    let query = insert_into(account_tokens)
        .values(form)
        .returning(action_token)
        .get_result(conn)
        .await?;

    Ok(AccountTokenCreateOutput {
        action_token: query,
    })
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountTokenViewInput {
    pub account_id: Uuid,
    pub action_token: Uuid,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountTokenViewOutput {
    pub action_type: TokenAction,
    pub token_data: serde_json::Value,
    pub ip_address: IpNet,
    pub user_agent: String,
}

/// Flags the action token as used and returns the action data.
///
/// # Tables
///
/// - account_emails
pub async fn consume_action_token(
    conn: &mut AsyncPgConnection,
    form: AccountTokenViewInput,
) -> DatabaseResult<AccountTokenViewOutput> {
    use schema::account_tokens::dsl::*;

    let filter_cond = account_id
        .eq(form.account_id)
        .and(action_token.eq(form.action_token))
        .and(used_at.is_null());

    let query = update(account_tokens.filter(filter_cond))
        .set(used_at.eq(now))
        .returning((action_type, token_data, ip_address, user_agent))
        .get_result(conn)
        .await?;

    Ok(query)
}
