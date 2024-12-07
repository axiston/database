//! Includes all embedded migrations.

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

/// Migration source that embeds migrations into the final binary.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./../../migrations");
