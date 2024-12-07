//! Custom database connection pool configurations.

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configures [`ConnectionPool`] for one or more gateways.
///
/// [`ConnectionPool`]: crate::Database
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[must_use = "configs do nothing unless you use them"]
pub struct DatabaseConfig {
    pub max_conn: Option<usize>,
    pub create_timeout: Option<Duration>,
    pub wait_timeout: Option<Duration>,
    pub recycle_timeout: Option<Duration>,
}

impl DatabaseConfig {
    /// Creates a new [`DatabaseConfig`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::max_conn`.
    pub fn with_max_conn(mut self, max_conn: usize) -> Self {
        self.max_conn = Some(max_conn);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::create_timeout`.
    pub fn with_create_timeout(mut self, create_timeout: Duration) -> Self {
        self.create_timeout = Some(create_timeout);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::wait_timeout`.
    pub fn with_wait_timeout(mut self, wait_timeout: Duration) -> Self {
        self.wait_timeout = Some(wait_timeout);
        self
    }

    /// Overwrites the default value of [`DatabaseConfig`]`::recycle_timeout`.
    pub fn with_recycle_timeout(mut self, recycle_timeout: Duration) -> Self {
        self.recycle_timeout = Some(recycle_timeout);
        self
    }

    /// Creates a new [`DatabaseConfig`] for a single gateway.
    pub fn new_single_gateway() -> Self {
        Self::default().with_max_conn(64)
    }

    /// Creates a new [`DatabaseConfig`] for multiple gateways.
    pub fn new_multiple_gateways() -> Self {
        Self::default().with_max_conn(8)
    }
}

#[cfg(test)]
mod test {
    use crate::{DatabaseConfig, DatabaseResult};

    #[test]
    fn single_gateway() -> DatabaseResult<()> {
        let _ = DatabaseConfig::new_single_gateway();
        Ok(())
    }

    #[test]
    fn multiple_gateways() -> DatabaseResult<()> {
        let _ = DatabaseConfig::new_multiple_gateways();
        Ok(())
    }
}
