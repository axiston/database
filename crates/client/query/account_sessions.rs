use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountSessionForm {
    pub region_id: String,
    pub ip_address: IpNet,
    pub user_agent: String,
}

/// - Creates the new session.
/// - Returns the token sequence.
///
/// # Tables
///
/// - account_sessions
pub async fn create_session(
    conn: &mut AsyncPgConnection,
    affected_account_id: Uuid,
    session_form: AccountSessionForm,
) -> DatabaseResult<Uuid> {
    use schema::account_sessions::dsl::*;

    let query = insert_into(account_sessions)
        .values((
            account_id.eq(affected_account_id),
            region_id.eq(session_form.region_id),
            ip_address.eq(session_form.ip_address),
            user_agent.eq(session_form.user_agent),
        ))
        .returning(token_seq)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// - Returns the active session.
///
/// # Tables
///
/// - account_sessions
pub async fn find_active_session(
    conn: &mut AsyncPgConnection,
    affected_account_id: Uuid,
    affected_token_seq: Uuid,
) -> DatabaseResult<Option<AccountSessionForm>> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(affected_account_id)
        .and(token_seq.eq(affected_token_seq))
        .and(expired_at.le(now))
        .and(deleted_at.is_null());

    let query = account_sessions
        .filter(filter_cond)
        .select(AccountSessionForm::as_select())
        .get_result(conn)
        .await
        .optional()?;

    Ok(query)
}

/// - Returns all active sessions.
///
/// # Tables
///
/// - account_sessions
pub async fn view_sessions(
    conn: &mut AsyncPgConnection,
    affected_account_id: Uuid,
) -> DatabaseResult<Vec<AccountSessionForm>> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(affected_account_id)
        .and(expired_at.le(now))
        .and(deleted_at.is_null());

    let query = account_sessions
        .filter(filter_cond)
        .select(AccountSessionForm::as_select())
        .get_results(conn)
        .await?;

    Ok(query)
}

/// - Deletes a single active session.
///
/// # Tables
///
/// - account_sessions
pub async fn delete_session(
    conn: &mut AsyncPgConnection,
    affected_account_id: Uuid,
    affected_token_seq: Uuid,
) -> DatabaseResult<()> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(affected_account_id)
        .and(token_seq.eq(affected_token_seq))
        .and(deleted_at.is_null());

    let _query = update(account_sessions.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}

/// - Deletes all active sessions except one.
///
/// # Tables
///
/// - account_sessions
pub async fn delete_sessions(
    conn: &mut AsyncPgConnection,
    affected_account_id: Uuid,
    token_seq_exception: Uuid,
) -> DatabaseResult<()> {
    use schema::account_sessions::dsl::*;

    let filter_cond = account_id
        .eq(affected_account_id)
        .and(token_seq.ne(token_seq_exception))
        .and(deleted_at.is_null());

    let _query = update(account_sessions.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}
