use axiston_db_schema::enumerations::EmailTypeForm;
use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::account_actions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountEmailsCreateInput<'a> {
    pub email_address: &'a str,
    pub action_type: EmailTypeForm,
}

/// - Creates the new action token.
/// - Returns the new action token.
///
/// # Tables
///
/// - account_emails
pub async fn create_email(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
    form: AccountEmailsCreateInput<'_>,
) -> DatabaseResult<Uuid> {
    use schema::account_actions::dsl::*;

    let query = insert_into(account_actions)
        .values((
            account_id.eq(account_id_recv),
            email_address.eq(form.email_address),
            action_type.eq(form.action_type),
        ))
        .returning(action_token)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// - Flags the action token as used.
/// - Returns the action type of the used action.
///
/// # Tables
///
/// - account_emails
pub async fn use_email(
    conn: &mut AsyncPgConnection,
    account_id_recv: Uuid,
    action_token_recv: Uuid,
) -> DatabaseResult<Option<EmailTypeForm>> {
    use schema::account_actions::dsl::*;

    let filter_cond = account_id
        .eq(account_id_recv)
        .and(action_token.eq(action_token_recv))
        .and(used_at.is_null());

    let query = update(account_actions.filter(filter_cond))
        .set(used_at.eq(now))
        .returning(action_type)
        .get_result(conn)
        .await
        .optional()?;

    Ok(query)
}
