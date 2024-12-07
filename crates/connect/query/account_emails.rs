use diesel::dsl::{insert_into, now, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::{Database, DatabaseResult};

#[derive(Debug, Clone, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EmailType"]
pub enum EmailTypeForm {
    #[db_rename = "confirm_email"]
    ConfirmEmail,
    #[db_rename = "update_email"]
    UpdateEmail,
    #[db_rename = "reset_password"]
    ResetPassword,
}

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::account_emails)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountEmailsForm {
    pub email_address: String,
    pub action_type: EmailTypeForm,
}

/// Queries for the `account_emails` table.
pub trait AccountEmailsExt {
    /// - Creates a new action token.
    /// - Returns a new action token.
    ///
    /// # Tables
    ///
    /// - account_emails
    async fn create_email(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountEmailsForm,
    ) -> DatabaseResult<Uuid>;

    /// - Flags the action token as used.
    /// - Returns the action type of the used action.
    ///
    /// # Tables
    ///
    /// - account_emails
    async fn use_email(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        action_token_recv: Uuid,
    ) -> DatabaseResult<Option<EmailTypeForm>>;
}

impl AccountEmailsExt for Database {
    async fn create_email(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountEmailsForm,
    ) -> DatabaseResult<Uuid> {
        use crate::schema::account_emails::dsl::*;

        let query = insert_into(account_emails)
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

    async fn use_email(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        action_token_recv: Uuid,
    ) -> DatabaseResult<Option<EmailTypeForm>> {
        use crate::schema::account_emails::dsl::*;

        let filter_cond = account_id
            .eq(account_id_recv)
            .and(action_token.eq(action_token_recv))
            .and(used_at.is_not_null());

        let query = update(account_emails.filter(filter_cond))
            .set(used_at.eq(now))
            .returning(action_type)
            .get_result(conn)
            .await
            .optional()?;

        Ok(query)
    }
}
