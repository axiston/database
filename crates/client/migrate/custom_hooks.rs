//! Includes all callbacks and hooks for [`DatabaseExt`].
//!
//! [`DatabaseExt`]: crate::DatabaseExt

use diesel_async::AsyncPgConnection;

use crate::DatabaseResult;

pub async fn pre_migrate(_conn: &mut AsyncPgConnection) -> DatabaseResult<()> {
    tracing::trace!(target: "database", "pre_migrate hook is running");

    Ok(())
}

pub async fn post_migrate(_conn: &mut AsyncPgConnection) -> DatabaseResult<()> {
    tracing::trace!(target: "database", "post_migrate hook is running");

    Ok(())
}
