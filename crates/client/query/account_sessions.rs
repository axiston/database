//! Data layer for account sessions management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountSession {
    pub region_id: String,
    pub ip_address: IpNet,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSessionToken {
    pub account_id: Uuid,
    pub token_seq: Uuid,
}

/// Creates the new session and returns the token sequence.
///
/// # Tables
///
/// - account_sessions
pub async fn create_session(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
    form: AccountSession,
) -> DatabaseResult<AccountSessionToken> {
    use schema::account_sessions::dsl::*;

    let query = insert_into(account_sessions)
        .values((
            account_id.eq(form_account_id),
            region_id.eq(form.region_id),
            ip_address.eq(form.ip_address),
            user_agent.eq(form.user_agent),
        ))
        .returning(token_seq)
        .get_result(conn)
        .await?;

    Ok(AccountSessionToken {
        account_id: form_account_id,
        token_seq: query,
    })
}

/// Returns the active session.
///
/// # Tables
///
/// - account_sessions
pub async fn find_active_session(
    conn: &mut AsyncPgConnection,
    form: AccountSessionToken,
) -> DatabaseResult<Option<AccountSession>> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(form.account_id)
        .and(token_seq.eq(form.token_seq))
        .and(expired_at.le(now))
        .and(deleted_at.is_null());

    let query = account_sessions
        .filter(filter_cond)
        .select(AccountSession::as_select())
        .get_result(conn)
        .await
        .optional()?;

    Ok(query)
}

/// Returns all active sessions.
///
/// # Tables
///
/// - account_sessions
pub async fn view_active_sessions(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
) -> DatabaseResult<Vec<AccountSession>> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(form_account_id)
        .and(expired_at.le(now))
        .and(deleted_at.is_null());

    let query = account_sessions
        .filter(filter_cond)
        .select(AccountSession::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}

/// Deletes a single active session.
///
/// # Tables
///
/// - account_sessions
pub async fn delete_session(
    conn: &mut AsyncPgConnection,
    form: AccountSessionToken,
) -> DatabaseResult<()> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(form.account_id)
        .and(token_seq.eq(form.token_seq))
        .and(deleted_at.is_null());

    let _query = update(account_sessions.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}

/// Deletes all active sessions except one.
///
/// # Tables
///
/// - account_sessions
pub async fn delete_sessions(
    conn: &mut AsyncPgConnection,
    form_account_id: Uuid,
    form_except_token_seq: Uuid,
) -> DatabaseResult<()> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(form_account_id)
        .and(token_seq.ne(form_except_token_seq))
        .and(deleted_at.is_null());

    let _query = update(account_sessions.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
