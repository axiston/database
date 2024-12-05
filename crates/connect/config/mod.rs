//! TODO.
//!

mod constraints;
mod custom_hooks;
mod pool_configs;

use std::fmt;

use derive_more::From;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub use crate::config::constraints::ConstraintViolation;
use crate::config::pool_configs::ConnectOptionsExt;
use crate::Result;

/// Contains a preconfigured `Postgres` database connection pool.
#[derive(Clone, From)]
pub struct AppDatabase {
    inner: DatabaseConnection,
}

impl AppDatabase {
    /// Returns a new [`AppDatabase`] connection.
    #[inline]
    pub fn new(inner: DatabaseConnection) -> Self {
        Self { inner }
    }

    /// Connects to the database and returns a new [`AppDatabase`].
    pub async fn connect<C: Into<ConnectOptions>>(connect_options: C) -> Result<Self> {
        let conn = Database::connect(connect_options).await;
        conn.map(Into::into).map_err(Into::into)
    }

    /// Connects to the database configured for a single gateway.
    pub async fn connect_single_instance<C: AsRef<str>>(addr: C) -> Result<Self> {
        Self::connect(ConnectOptions::new_single_instance(addr.as_ref())).await
    }

    /// Connects to the database configured for multiple gateways.
    pub async fn connect_multiple_instances<C: AsRef<str>>(addr: C) -> Result<Self> {
        Self::connect(ConnectOptions::new_multiple_instances(addr.as_ref())).await
    }

    /// Returns the underlying database connection.
    #[inline]
    pub fn database_connection(&self) -> DatabaseConnection {
        self.inner.clone()
    }

    /// Returns a reference to the underlying database connection.
    #[inline]
    pub fn as_database_connection(&self) -> &DatabaseConnection {
        &self.inner
    }
}

impl fmt::Debug for AppDatabase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Database").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::{AppDatabase, Result};

    #[tokio::test]
    async fn single_instance() -> Result<()> {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let _ = AppDatabase::connect_single_instance(addr).await?;
        Ok(())
    }

    #[test]
    async fn multiple_instances() -> Result<()> {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let _ = AppDatabase::connect_multiple_instances(addr).await?;
        Ok(())
    }
}
