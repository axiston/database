### axiston/database

[![Build Status][action-badge]][action-url]
[![DockerHub][docker-badge]][docker-url]

**Check out other `axiston` projects [here](https://github.com/axiston).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/database/build.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/database/actions/workflows/build.yaml
[docker-badge]: https://img.shields.io/docker/automated/_/postgres?style=flat-square&logo=docker&logoColor=white&color=%232496ED
[docker-url]: https://hub.docker.com/u/axiston/database

Unmodified Postgres 17 Docker image with all extensions and migrations.

#### Notes

- Includes [pg_cron][pg_cron] `postgres` extension.
- Consists out of [axiston_db_migrate][migrate] and [axiston_db_schema][schema]
  crates.

[migrate]: https://crates.io/crates/axiston-db-migrate
[schema]: https://crates.io/crates/axiston-db-schema

#### Dependencies

- Depends on [diesel][diesel] as an object–relational mapping and
  [diesel_migrations][diesel_migrations] as a migration tool.
- Depends on [diesel_async][diesel_async] for asynchronous connections and
  [diesel_derive_enum][diesel_derive_enum] for enum boilerplate.

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
