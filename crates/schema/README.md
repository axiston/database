### database/schema

[![Build Status][action-badge]][action-url]
[![Crate Docs][docs-badge]][docs-url]
[![Crate Version][crates-badge]][crates-url]

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/database/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/database/actions/workflows/build.yaml
[crates-badge]: https://img.shields.io/crates/v/axiston-db-schema.svg?logo=rust&style=flat-square
[crates-url]: https://crates.io/crates/axiston-db-schema
[docs-badge]: https://img.shields.io/docsrs/axiston-db-schema?logo=Docs.rs&style=flat-square
[docs-url]: http://docs.rs/axiston-db-schema

**Check out other `axiston` projects [here](https://github.com/axiston).**

Contains the database schema for the application, generated using
[diesel_cli][diesel_cli].

The schema is updated automatically after all migrations are applied during a
GitHub Action triggered by a pull request to the `main` branch. For instructions
on how to regenerate the schema locally, refer to the `Makefile` and `build.rs`
files in the project root.

[diesel_cli]: https://crates.io/crates/diesel_cli
