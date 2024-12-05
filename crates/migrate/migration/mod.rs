use sea_orm::sea_query::{Alias, IntoIden};
use sea_orm::{DatabaseConnection, DynIden};
use sea_orm_migration::{MigrationTrait, MigratorTrait};

use crate::Result;

mod m20241205_021500_init;
mod m20241205_021503_init;

/// Implements the core migrator of the application.
#[derive(Debug, Clone, Default)]
pub struct AppMigrator;

#[async_trait::async_trait]
impl MigratorTrait for AppMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_021500_init::Migration),
            Box::new(m20241205_021503_init::Migration),
        ]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("migrations").into_iden()
    }
}

/// Extends [`DatabaseConnection`] with migration utilities.
pub trait DatabaseConnectionExt {
    /// Applies `steps` pending migrations.
    async fn apply_migrations(&self, steps: Option<u32>) -> Result<()>;

    /// Rolls back `steps` pending migrations.
    async fn rollback_migrations(&self, steps: Option<u32>) -> Result<()>;
}

impl DatabaseConnectionExt for DatabaseConnection {
    async fn apply_migrations(&self, steps: Option<u32>) -> Result<()> {
        AppMigrator::up(self, steps).await?;
        Ok(())
    }

    async fn rollback_migrations(&self, steps: Option<u32>) -> Result<()> {
        AppMigrator::down(self, steps).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{ConnectOptions, Database};

    use crate::{DatabaseConnectionExt, Result};

    #[tokio::test]
    async fn migrations() -> Result<()> {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let opts = ConnectOptions::new(addr);

        let conn = Database::connect(opts).await?;
        conn.apply_migrations(None).await?;
        conn.rollback_migrations(None).await?;
        Ok(())
    }
}
