use diesel::dsl::{insert_into, now, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

/// Queries for the `project_invites` table.
pub trait ProjectInvitesExt {
    /// - Sends a new invitation.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn send_invite(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_send: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Cancels the pending invite.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn cancel_invite(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_send: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Accepts the pending invite.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn accept_invite(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Declines the pending invite.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn decline_invite(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Lists all invites of the project.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn view_project_invites(&self, project_id: Uuid) -> DatabaseResult<()>;

    /// - Lists all invites of the account.
    ///
    /// # Tables
    ///
    /// - project_invites
    async fn view_account_invites(&self, account_id: Uuid) -> DatabaseResult<()>;
}

// impl ProjectInvitesExt for Database {}
