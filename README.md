Introduction
============

Basic template for web service in Rust. Uses `sqlx` for database connectivity.

This template contains a basic Dhall configuration which can be used to generate
a `.env` file and a `docker-compose` configuration. to do this, run:

```bash
make dev
```

Requirements
------------
To handle migrations etc, `sqlx-cli` can be utilized. Install it via:

```bash
cargo install sqlx-cli --no-default-features --features=postgres
```

Environment
-----------
The `sqlx` CLI tool requires the environment variable `DATABASE_URL` to be set
to the connection string. For example:

```bash
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
```

This is done in `.env`.

Creating a migration
--------------------
A new migration can be created by running:

```bash
sqlx migrate add create_persons_table
```

Run the migration(s) with:

```bash
sqlx migrate run
```