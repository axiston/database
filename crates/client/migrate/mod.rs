//! Asynchronous `postgres` migrator extension.

mod custom_hooks;

use std::ops::DerefMut;

use axiston_db_schema::MIGRATIONS;
use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_migrations::MigrationHarness;
use tokio::task::spawn_blocking;

use crate::migrate::custom_hooks::{post_migrate, pre_migrate};
use crate::{Database, DatabaseError, DatabaseResult};

/// Asynchronous `postgres` migrator extension.
///
/// - Implemented with [`diesel`] and [`deadpool`].
pub trait DatabaseExt {
    /// Executes all pending migrations from [`MIGRATIONS`].
    async fn apply_migrations(&self) -> DatabaseResult<u64>;

    /// Reverts all applied migrations from [`MIGRATIONS`].
    async fn rollback_migrations(&self) -> DatabaseResult<u64>;
}

impl DatabaseExt for Database {
    async fn apply_migrations(&self) -> DatabaseResult<u64> {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        pre_migrate(wrapper.deref_mut()).await?;

        let migrations: DatabaseResult<u64> = spawn_blocking(move || {
            let versions = wrapper
                .run_pending_migrations(MIGRATIONS)
                .map_err(DatabaseError::Migration)?;
            Ok(versions.len() as u64)
        })
        .await
        .unwrap();

        let mut conn = self.get_connection().await?;
        post_migrate(conn.deref_mut()).await?;
        migrations
    }

    async fn rollback_migrations(&self) -> DatabaseResult<u64> {
        let conn = self.get_connection().await?;
        let mut wrapper: AsyncConnectionWrapper<_> = conn.into();
        pre_migrate(wrapper.deref_mut()).await?;

        let migrations: DatabaseResult<u64> = spawn_blocking(move || {
            let versions = wrapper
                .revert_all_migrations(MIGRATIONS)
                .map_err(DatabaseError::Migration)?;
            Ok(versions.len() as u64)
        })
        .await
        .unwrap();

        let mut conn = self.get_connection().await?;
        post_migrate(conn.deref_mut()).await?;
        migrations
    }
}

#[cfg(test)]
mod test {
    use crate::{Database, DatabaseExt, DatabaseResult};

    async fn create_database_client() -> DatabaseResult<Database> {
        let addr = "postgresql://postgres:postgres@localhost:5432/postgres";
        let database = Database::new_single_gateway(addr);
        let _ = database.get_connection().await?;
        Ok(database)
    }

    #[tokio::test]
    async fn apply_migrations() -> DatabaseResult<()> {
        let database = create_database_client().await?;
        let _ = database.apply_migrations().await?;
        Ok(())
    }

    #[tokio::test]
    async fn rollback_migrations() -> DatabaseResult<()> {
        let database = create_database_client().await?;
        let _ = database.rollback_migrations().await?;
        Ok(())
    }
}
