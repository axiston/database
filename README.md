### axiston/database

[![Build Status][action-badge]][action-url]
[![DockerHub][docker-badge]][docker-url]

**Check out other `axiston` projects [here](https://github.com/axiston).**

[action-badge]: https://img.shields.io/github/actions/workflow/status/axiston/database/docker.yaml?branch=main&label=build&logo=github&style=flat-square
[action-url]: https://github.com/axiston/database/actions/workflows/docker.yaml
[docker-badge]: https://img.shields.io/docker/automated/_/postgres?style=flat-square&logo=docker&logoColor=white&color=%232496ED
[docker-url]: https://hub.docker.com/u/axiston/database

A standard PostgreSQL 17 Docker image with all project-required extensions,
migrations, and a database client including a generated schema.

#### Notes

- See the [official Postgres image] for more details on PostgreSQL configuration
  and features.
- If you already have a running PostgreSQL instance that you want to use with
  this service, you must install, configure, and enable the required extensions
  before starting the gateway. Refer to the [Dockerfile][dockerfile] for
  details.

[official Postgres image]: https://hub.docker.com/_/postgres
[pg_cron]: https://github.com/citusdata/pg_cron
[dockerfile]: ./Dockerfile
