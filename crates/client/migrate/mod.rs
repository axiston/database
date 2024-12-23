//! TODO.

use axiston_db_schema::MIGRATIONS;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_migrations::MigrationHarness;

use crate::{Database, DatabaseError, DatabaseResult};

/// Asynchronous `postgres` migrator.
///
/// - Implemented with [`diesel`] and [`deadpool`].
pub trait DatabaseExt {
    /// Executes all pending migrations from [`MIGRATIONS`].
    async fn apply_migrations(&mut self) -> DatabaseResult<u64>;

    /// Reverts all applied migrations from [`MIGRATIONS`].
    async fn rollback_migrations(&mut self) -> DatabaseResult<u64>;
}

impl DatabaseExt for Database {
    async fn apply_migrations(&mut self) -> DatabaseResult<u64> {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        let versions = wrapper
            .run_pending_migrations(MIGRATIONS)
            .map_err(DatabaseError::Migration)?;
        Ok(versions.len() as u64)
    }

    async fn rollback_migrations(&mut self) -> DatabaseResult<u64> {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        let versions = wrapper
            .revert_all_migrations(MIGRATIONS)
            .map_err(DatabaseError::Migration)?;
        Ok(versions.len() as u64)
    }
}

#[cfg(test)]
mod test {
    use diesel_async::pooled_connection::deadpool::Pool;
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::AsyncPgConnection;
    use tokio::runtime::Runtime;

    use crate::{DatabaseExt, DatabaseResult};

    fn get_connection_pool() -> Pool<AsyncPgConnection> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let conn_manager = AsyncDieselConnectionManager::new(addr);
        let pool = Pool::builder(conn_manager);
        pool.build().expect("should not require runtime")
    }

    #[test]
    fn apply_migrations() -> DatabaseResult<()> {
        let pool = get_connection_pool();
        let rt = Runtime::new().expect("should not panic");
        let conn = rt.block_on(async { pool.get().await.unwrap() });

        let mut migrator = DatabaseMigrator::new(conn);
        let _ = migrator.apply_migrations()?;
        Ok(())
    }

    #[test]
    fn rollback_migrations() -> DatabaseResult<()> {
        let pool = get_connection_pool();
        let rt = Runtime::new().expect("should not panic");
        let conn = rt.block_on(async { pool.get().await.unwrap() });

        let mut migrator = DatabaseMigrator::new(conn);
        let _ = migrator.rollback_migrations()?;
        Ok(())
    }
}
