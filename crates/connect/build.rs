#![forbid(unsafe_code)]

use tokio::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // # https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/

    let cmd = std::env::var("SEA_ORM_CLI").unwrap_or_else(|_| "sea-orm-cli".to_owned());
    let schema = std::env::var("DATABASE_SCHEMA").unwrap_or_else(|_| "public".to_owned());
    let addr = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://usr:pwd@localhost:5432/db".to_owned());

    let mut cmd = Command::new(cmd)
        .args(["generate", "entity"])
        .args(["--database-url", &addr])
        .args(["--database-schema", &schema])
        .args(["--output-dir", "./entity"])
        .args(["--date-time-crate", "time"])
        .args(["--with-serde", "both"])
        .arg("--with-copy-enums")
        .arg("--expanded-format")
        .arg("--verbose")
        .spawn()?;

    cmd.wait().await?;
    Ok(())
}
