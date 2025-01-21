#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub use crate::types::{constraints, enumerations};

pub mod schema;
mod types;

/// Migration source that embeds migrations into the final binary.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
