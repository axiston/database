### axiston/database

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

Unmodified Postgres 17 Docker image with all extensions and migrations.

#### Notes

- Includes [pg_cron][pg_cron] `postgres` extension.

#### Dependencies

- Depends on [diesel][diesel] as an object–relational mapping and
  [diesel_migrations][diesel_migrations] as a migration tool.
- Depends on community crates [diesel_async][diesel_async] for asynchronous
  connections and [diesel_derive_enum][diesel_derive_enum] for enum boilerplate.

[pg_cron]: https://github.com/citusdata/pg_cron
[diesel]: https://crates.io/crates/diesel
[diesel_migrations]: https://crates.io/crates/diesel_migrations
[diesel_async]: https://crates.io/crates/diesel-async/
[diesel_derive_enum]: https://crates.io/crates/diesel-derive-enum

#### Guidelines

- Migrations are append-only. Once a migration is merged into the `main` branch,
  do not modify it.
- Migrations in `migration/` must be idempotent, ensuring they can be run
  multiple times without causing issues.
- Self-hosted service users should update role passwords separately after
  running all migrations.
- Production releases are done by publishing a new GitHub release from the
  `main` branch.
