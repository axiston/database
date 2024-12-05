use axiston_database_migrate::AppMigrator;
use sea_orm_migration::cli::run_cli;

#[tokio::main]
async fn main() {
    run_cli(AppMigrator).await;
}
