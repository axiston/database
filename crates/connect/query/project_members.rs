use diesel::dsl::{insert_into, now, update};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

/// Queries for the `project_members` table.
pub trait ProjectMembersExt {
    /// - Lists all projects.
    ///
    /// # Tables
    ///
    /// - projects
    /// - project_members
    async fn view_projects(
        &self,
        conn: &mut AsyncPgConnection,
        account_id: Uuid,
    ) -> DatabaseResult<()>;

    /// - Lists all active members of the project.
    ///
    /// # Tables
    ///
    /// - project_members
    async fn view_members(
        &self,
        conn: &mut AsyncPgConnection,
        project_id: Uuid,
    ) -> DatabaseResult<()>;

    /// - Kicks an active member from the project.
    ///
    /// # Tables
    ///
    /// - project_members
    async fn delete_member(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_send: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// - Leaves the project (same as kicking self).
    ///
    /// # Tables
    ///
    /// - project_members
    async fn leave_project(
        &self,
        conn: &mut AsyncPgConnection,
        project_id_recv: Uuid,
        account_id_send: Uuid,
    ) -> DatabaseResult<()> {
        self.delete_member(conn, project_id_recv, account_id_send, account_id_send)
            .await
    }
}

// impl ProjectMembersExt for Database {}
