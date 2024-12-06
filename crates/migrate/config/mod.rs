//! Asynchronous `postgres` migrator and its configuration.

mod custom_hooks;
mod embedded_query;

use std::fmt;

use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use diesel_migrations::MigrationHarness;

use crate::config::custom_hooks::MIGRATIONS;
use crate::DatabaseMigratorResult;

/// Asynchronous `postgres` migrator.
///
/// - Implemented with [`diesel`] and [`deadpool`].
pub struct DatabaseMigrator {
    conn: AsyncConnectionWrapper<Object<AsyncPgConnection>>,
}

impl DatabaseMigrator {
    /// Returns a new [`DatabaseMigrator`].
    #[inline]
    pub fn new(conn: Object<AsyncPgConnection>) -> Self {
        Self { conn: conn.into() }
    }

    /// Applies all pending migrations.
    pub fn apply_migrations(&mut self) -> DatabaseMigratorResult<()> {
        let _ = self.conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    /// Rolls back all migrations.
    pub fn rollback_migrations(&mut self) -> DatabaseMigratorResult<()> {
        let _ = self.conn.revert_all_migrations(MIGRATIONS)?;
        Ok(())
    }
}

impl fmt::Debug for DatabaseMigrator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseMigrator").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use diesel_async::pooled_connection::deadpool::Object;
    use diesel_async::AsyncPgConnection;

    use crate::{DatabaseMigrator, DatabaseMigratorResult};

    async fn get_connection() -> DatabaseMigratorResult<Object<AsyncPgConnection>> {
        todo!()
    }

    #[tokio::test]
    async fn apply_migrations() -> DatabaseMigratorResult<()> {
        let conn = get_connection().await?;
        let mut migrator = DatabaseMigrator::new(conn);
        migrator.apply_migrations().await?;
        Ok(())
    }

    #[tokio::test]
    async fn rollback_migrations() -> DatabaseMigratorResult<()> {
        let conn = get_connection().await?;
        let mut migrator = DatabaseMigrator::new(conn);
        migrator.rollback_migrations().await?;
        Ok(())
    }
}
