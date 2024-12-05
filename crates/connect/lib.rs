#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_database_connect::{Result, AppDatabase};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let addr = "postgresql://usr:pwd@localhost:5432/db";
//!     let conn = AppDatabase::connect_single_instance(addr).await;
//!     Ok(())
//! }
//! ```

use derive_more::{Deref, DerefMut, From};
use sea_orm::DbErr;

pub use crate::config::{AppDatabase, ConstraintViolation};

mod config;
mod entity;
mod query;

/// Unrecoverable failure of the [`AppDatabase`].
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

    /// Parses a constraint violation from the underlying error.
    pub fn constraint(&self) -> Option<ConstraintViolation> {
        self.inner.sql_err().and_then(ConstraintViolation::new)
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
