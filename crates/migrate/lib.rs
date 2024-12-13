#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_db_migrate::{DatabaseMigratorResult, DatabaseMigrator};
//! use diesel_async::AsyncPgConnection;
//! use diesel_async::pooled_connection::deadpool::Object;
//!
//! fn get_connection() -> Object<AsyncPgConnection> {
//!     todo!()
//! }
//!
//! #[tokio::main]
//! async fn main() -> DatabaseMigratorResult<()> {
//!     let mut x = DatabaseMigrator::new(get_connection());
//!     x.apply_migrations().await?;
//!     Ok(())
//! }
//! ```


use derive_more::{Deref, DerefMut, From};

pub use crate::config::DatabaseMigrator;

mod config;

/// Type-erased [`Error`] type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Unrecoverable failure of the [`DatabaseMigrator`].
///
/// Includes all error types that may occur.
#[derive(Debug, From, Deref, DerefMut, thiserror::Error)]
#[error("underlying sql driver failure: {inner}")]
#[must_use = "errors do nothing unless you use them"]
pub struct DatabaseMigratorError {
    inner: BoxError,
}

impl DatabaseMigratorError {
    /// Returns a new [`DatabaseMigratorError`].
    #[inline]
    pub fn new(inner: BoxError) -> Self {
        Self { inner }
    }

    /// Returns the underlying database error.
    #[inline]
    pub fn into_inner(self) -> BoxError {
        self.inner
    }
}

/// Specialized [`Result`] alias for the [`DatabaseMigratorError`] type.
pub type DatabaseMigratorResult<T, E = DatabaseMigratorError> = Result<T, E>;
