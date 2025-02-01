### database/client

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

**Check out other `axiston` projects [here](https://github.com/axiston).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/database/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/database/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/axiston-database-connect.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/axiston-database-connect
[docs-badge]: https://img.shields.io/docsrs/axiston-database-connect?logo=Docs.rs&style=flat-square
[docs-url]: http://docs.rs/axiston-database-connect

Provides a database client for the application, integrating key features such as
database connection pooling powered by [deadpool][deadpool], migrations (and
rollbacks), predefined queries for database entities, and tracing support for
debugging and observability.

[deadpool]: https://crates.io/crates/deadpool
