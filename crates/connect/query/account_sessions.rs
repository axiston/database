use diesel::dsl::{insert_into, now, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use ipnet::IpNet;
use uuid::Uuid;

use crate::{Database, DatabaseResult};

#[derive(Debug, Clone, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::account_sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct AccountSessionForm {
    pub region_id: String,
    pub ip_address: IpNet,
    pub user_agent: String,
}

/// Queries for the `account_sessions` table.
pub trait AccountSessionsExt {
    /// - Creates the new session.
    /// - Returns the token sequence.
    ///
    /// # Tables
    ///
    /// - account_sessions
    async fn create_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountSessionForm,
    ) -> DatabaseResult<Uuid>;

    /// - Returns the active session.
    ///
    /// # Tables
    ///
    /// - account_sessions
    async fn find_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_recv: Uuid,
    ) -> DatabaseResult<Option<AccountSessionForm>>;

    /// - Returns all active sessions.
    ///
    /// # Tables
    ///
    /// - account_sessions
    async fn view_sessions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<Vec<AccountSessionForm>>;

    /// - Deletes a single active session.
    ///
    /// # Tables
    ///
    /// - account_sessions
    async fn delete_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Deletes all active sessions except one.
    ///
    /// # Tables
    ///
    /// - account_sessions
    async fn delete_sessions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_send: Uuid,
    ) -> DatabaseResult<()>;
}

impl AccountSessionsExt for Database {
    async fn create_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        form: AccountSessionForm,
    ) -> DatabaseResult<Uuid> {
        use crate::schema::account_sessions::dsl::*;

        let query = insert_into(account_sessions)
            .values((
                account_id.eq(account_id_recv),
                region_id.eq(form.region_id),
                ip_address.eq(form.ip_address),
                user_agent.eq(form.user_agent),
            ))
            .returning(token_seq)
            .get_result(conn)
            .await?;

        Ok(query)
    }

    async fn find_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_recv: Uuid,
    ) -> DatabaseResult<Option<AccountSessionForm>> {
        use crate::schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(account_id_recv)
            .and(token_seq.eq(token_seq_recv))
            .and(deleted_at.is_not_null());

        let query = account_sessions
            .filter(filter_cond)
            .select(AccountSessionForm::as_select())
            .get_result(conn)
            .await
            .optional()?;

        Ok(query)
    }

    async fn view_sessions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
    ) -> DatabaseResult<Vec<AccountSessionForm>> {
        use crate::schema::account_sessions::dsl::*;

        let filter_cond = account_id.eq(account_id_recv).and(deleted_at.is_not_null());

        let query = account_sessions
            .filter(filter_cond)
            .select(AccountSessionForm::as_select())
            .get_results(conn)
            .await?;

        Ok(query)
    }

    async fn delete_session(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_recv: Uuid,
    ) -> DatabaseResult<()> {
        use crate::schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(account_id_recv)
            .and(token_seq.eq(token_seq_recv))
            .and(deleted_at.is_not_null());

        let _query = update(account_sessions.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn delete_sessions(
        &self,
        conn: &mut AsyncPgConnection,
        account_id_recv: Uuid,
        token_seq_send: Uuid,
    ) -> DatabaseResult<()> {
        use crate::schema::account_sessions::dsl::*;

        let filter_cond = account_id
            .eq(account_id_recv)
            .and(token_seq.ne(token_seq_send))
            .and(deleted_at.is_not_null());

        let _query = update(account_sessions.filter(filter_cond))
            .set(deleted_at.eq(now))
            .execute(conn)
            .await?;

        Ok(())
    }
}
