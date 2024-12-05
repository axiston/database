//! Custom configurations for [`ConnectOptions`].
//!

use std::time::Duration;

use sea_orm::ConnectOptions;

/// Extends [`ConnectOptions`] with preconfigured constructors.
pub trait ConnectOptionsExt {
    /// Returns [`ConnectOptions`] suitable for a setup with a single gateway.
    fn new_single_instance<C: Into<String>>(addr: C) -> ConnectOptions;

    /// Returns [`ConnectOptions`] suitable for a setup with a single gateway.
    fn new_multiple_instances<C: Into<String>>(addr: C) -> ConnectOptions;
}

impl ConnectOptionsExt for ConnectOptions {
    fn new_single_instance<C: Into<String>>(addr: C) -> ConnectOptions {
        let mut connection_options = ConnectOptions::new(addr);
        connection_options
            .idle_timeout(Duration::from_secs(8 * 60))
            .acquire_timeout(Duration::from_secs(40))
            .max_lifetime(Duration::from_secs(40 * 60))
            .min_connections(2)
            .max_connections(64);

        connection_options
    }

    fn new_multiple_instances<C: Into<String>>(addr: C) -> ConnectOptions {
        let mut connection_options = ConnectOptions::new(addr);
        connection_options
            .idle_timeout(Duration::from_secs(2 * 60))
            .acquire_timeout(Duration::from_secs(40))
            .max_lifetime(Duration::from_secs(20 * 60))
            .min_connections(0)
            .max_connections(32);

        connection_options
    }
}

#[cfg(test)]
mod test {
    use sea_orm::ConnectOptions;

    use crate::connect::ConnectOptionsExt;

    #[test]
    fn single_instance() {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let _ = ConnectOptions::new_single_instance(addr);
    }

    #[test]
    fn multiple_instances() {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let _ = ConnectOptions::new_multiple_instances(addr);
    }
}
