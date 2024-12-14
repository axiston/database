//! Asynchronous `postgres` migrator and its configuration.

mod custom_hooks;
mod embedded_query;

use std::fmt;

use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use diesel_migrations::MigrationHarness;

use crate::config::embedded_query::MIGRATIONS;
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
    pub fn apply_migrations(&mut self) -> DatabaseMigratorResult<usize> {
        let versions = self.conn.run_pending_migrations(MIGRATIONS)?;
        Ok(versions.len())
    }

    /// Rolls back all migrations.
    pub fn rollback_migrations(&mut self) -> DatabaseMigratorResult<usize> {
        let versions = self.conn.revert_all_migrations(MIGRATIONS)?;
        Ok(versions.len())
    }
}

impl fmt::Debug for DatabaseMigrator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseMigrator").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::AsyncPgConnection;
    use tokio::runtime::Runtime;

    use crate::{DatabaseMigrator, DatabaseMigratorResult};

    fn get_connection_pool() -> Pool<AsyncPgConnection> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let conn_manager = AsyncDieselConnectionManager::new(addr);
        let pool = Pool::builder(conn_manager);
        pool.build().expect("should not require runtime")
    }

    #[test]
    fn apply_migrations() -> DatabaseMigratorResult<()> {
        let pool = get_connection_pool();
        let rt = Runtime::new().expect("should not panic");
        let conn = rt.block_on(async { pool.get().await.unwrap() });

        let mut migrator = DatabaseMigrator::new(conn);
        let _ = migrator.apply_migrations()?;
        Ok(())
    }

    #[test]
    fn rollback_migrations() -> DatabaseMigratorResult<()> {
        let pool = get_connection_pool();
        let rt = Runtime::new().expect("should not panic");
        let conn = rt.block_on(async { pool.get().await.unwrap() });

        let mut migrator = DatabaseMigrator::new(conn);
        let _ = migrator.rollback_migrations()?;
        Ok(())
    }
}
