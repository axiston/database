#![forbid(unsafe_code)]
#![allow(async_fn_in_trait)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_db_client::{DatabaseResult, Database};
//!
//! #[tokio::main]
//! async fn main() -> DatabaseResult<()> {
//!     let addr = "postgresql://usr:pwd@localhost:5432/db";
//!     let _ = Database::new_single_gateway(addr);
//!     Ok(())
//! }
//! ```

use deadpool::managed::TimeoutType;
use diesel::result::{ConnectionError, Error};
use diesel_async::pooled_connection::deadpool::PoolError;
use diesel_async::pooled_connection::PoolError as PoolError2;

pub use crate::config::{Database, DatabaseConfig};
pub use crate::migrate::DatabaseExt;
pub use crate::query::*;
pub(crate) use crate::utils::*;

mod config;
mod migrate;
mod query;
mod utils;

/// Type-erased [`Error`] type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Unrecoverable failure of the [`Database`].
///
/// Includes all error types that may occur.
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum DatabaseError {
    /// [`deadpool::managed::PoolError::Timeout`].
    #[error("timeout error")]
    Timeout(TimeoutType),
    /// [`diesel_async::pooled_connection::PoolError::ConnectionError`]
    #[error("orm connection error: {0}")]
    Connection(ConnectionError),
    /// Errors returned by [`DatabaseExt`] and [`diesel_migrations::MigrationHarness`].
    #[error("orm migration error: {0}")]
    Migration(BoxError),
    /// [`diesel_async::pooled_connection::PoolError::QueryError`]
    #[error("orm query error: {0}")]
    Query(Error),
}

impl From<PoolError> for DatabaseError {
    fn from(value: PoolError) -> Self {
        match value {
            PoolError::Timeout(timeout_type) => Self::Timeout(timeout_type),
            PoolError::Backend(PoolError2::QueryError(query_error)) => Self::Query(query_error),
            PoolError::Backend(PoolError2::ConnectionError(connection_error)) => {
                Self::Connection(connection_error)
            }

            PoolError::PostCreateHook(_) => unreachable!("custom hooks should not return errors"),
            PoolError::NoRuntimeSpecified => unreachable!("tokio runtime should be specified"),
            PoolError::Closed => unreachable!("connection pool should not be closed"),
        }
    }
}

impl From<Error> for DatabaseError {
    #[inline]
    fn from(value: Error) -> Self {
        Self::Query(value)
    }
}

/// Specialized [`Result`] alias for the [`DatabaseError`] type.
pub type DatabaseResult<T, E = DatabaseError> = Result<T, E>;
