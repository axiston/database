### axiston/database

Unmodified Postgres 17 Docker image with all extensions and migrations.

Lorem Ipsum. Lorem Ipsum. Lorem Ipsum.

#### Notes

- Lorem Ipsum.
- Lorem Ipsum.
- Lorem Ipsum.

#### Guidelines

- Migrations are append-only. Once a migration is merged into the `main` branch,
  do not modify it.
- Migrations in `migration/` must be idempotent, ensuring they can be run
  multiple times without causing issues.
- Self-hosted Axiston users should update role passwords separately after
  running all migrations.
- Production releases are done by publishing a new GitHub release from the
  `main` branch.
