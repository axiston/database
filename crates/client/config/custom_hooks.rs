//! Includes all callbacks and hooks for [`diesel`] and [`deadpool`].

use deadpool::managed::{HookResult, Metrics};
use diesel::ConnectionResult;
use diesel_async::pooled_connection::PoolError;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::BoxFuture;
use futures::FutureExt;

/// Custom setup procedure used to establish a new connection.
///
/// See [`ManagerConfig`] and [`SetupCallback`] for more details.
///
/// [`ManagerConfig`]: diesel_async::pooled_connection::ManagerConfig
/// [`SetupCallback`]: diesel_async::pooled_connection::SetupCallback
pub fn setup_callback<C>(addr: &str) -> BoxFuture<ConnectionResult<C>>
where
    C: AsyncConnection + 'static,
{
    // tracing::trace!(target: "database", "setup_callback is running");
    C::establish(addr).boxed()
}

/// Custom hook called after a new connection has been established.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_create(_conn: &mut AsyncPgConnection, _metrics: &Metrics) -> HookResult<PoolError> {
    tracing::trace!(target: "database", "post_create hook is running");

    // Note: should never return an error.
    Ok(())
}

/// Custom hook called before a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn pre_recycle(_conn: &mut AsyncPgConnection, _metrics: &Metrics) -> HookResult<PoolError> {
    tracing::trace!(target: "database", "pre_recycle hook is running");

    // Note: should never return an error.
    Ok(())
}

/// Custom hook called after a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_recycle(_conn: &mut AsyncPgConnection, _metrics: &Metrics) -> HookResult<PoolError> {
    tracing::trace!(target: "database", "post_recycle hook is running");

    // Note: should never return an error.
    Ok(())
}
