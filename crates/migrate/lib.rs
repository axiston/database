#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_database_migrate::{Result, DatabaseConnectionExt};
//! use sea_orm::{ConnectOptions, Database};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let addr = "postgresql://usr:pwd@localhost:5432/db";
//!     let opts = ConnectOptions::new(addr);
//!     let conn = Database::connect(opts).await?;
//!     conn.apply_migrations(None).await?;
//!     conn.rollback_migrations(None).await?;
//!     Ok(())
//! }
//! ```

use derive_more::{Deref, DerefMut, From};
use sea_orm::DbErr;

mod config;
mod migration;

pub use crate::migration::{AppMigrator, DatabaseConnectionExt};

/// Unrecoverable failure of the [`AppMigrator`].
///
/// Includes all error types that may occur.
#[derive(Debug, From, Deref, DerefMut, thiserror::Error)]
#[error("underlying sql driver failure: {inner}")]
#[must_use = "errors do nothing unless you use them"]
pub struct Error {
    inner: DbErr,
}

impl Error {
    /// Returns a new [`Error`].
    #[inline]
    pub fn new(inner: DbErr) -> Self {
        Self { inner }
    }

    /// Returns the underlying database error.
    #[inline]
    pub fn into_inner(self) -> DbErr {
        self.inner
    }
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
