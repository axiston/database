use diesel::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{Database, DatabaseResult};

/// Queries for the `projects` table.
pub trait ProjectsExt {
    /// Creates a new project.
    ///
    /// # Tables
    ///
    /// - projects
    async fn create_project(&self, account_id_recv: Uuid) -> DatabaseResult<Uuid>;

    /// Updates project's metadata.
    ///
    /// # Tables
    ///
    /// - projects
    async fn update_project(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;

    /// Deletes an existing project.
    ///
    /// # Tables
    ///
    /// - projects
    async fn delete_project(
        &self,
        project_id_recv: Uuid,
        account_id_recv: Uuid,
    ) -> DatabaseResult<()>;
}

// impl ProjectsExt for Database {}
