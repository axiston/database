#![forbid(unsafe_code)]
#![allow(async_fn_in_trait)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_database_connect::{DatabaseResult, Database};
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

use crate::config::ConstraintViolation;
pub use crate::config::{Database, DatabaseConfig};
pub use crate::query::{account_queries, project_queries, workflow_queries};

mod config;
mod query;
mod schema;

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
    #[error("connection error: {0}")]
    Connection(ConnectionError),
    /// [`diesel_async::pooled_connection::PoolError::QueryError`]
    #[error("query error: {0}")]
    Query(Error),
}

impl DatabaseError {
    /// Returns a new [`DatabaseError`].
    #[inline]
    pub fn new(inner: PoolError) -> Self {
        match inner {
            PoolError::Timeout(timeout_type) => Self::Timeout(timeout_type),
            PoolError::Backend(PoolError2::ConnectionError(connection_error)) => {
                Self::Connection(connection_error)
            }
            PoolError::Backend(PoolError2::QueryError(query_error)) => Self::Query(query_error),
            PoolError::PostCreateHook(_) => unreachable!(),
            PoolError::NoRuntimeSpecified => unreachable!(),
            PoolError::Closed => unreachable!(),
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
