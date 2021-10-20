# Examples

By default, all examples in this directory require a postgres server, which you can run using `docker-compose.dev.yml` in the `examples/web` directory.

## Simple

This example initializes persistence, gets a connection and runs the query.

```sh
cargo run --example simple --features r2d2_postgres
```

## Transaction

This is extended simple example where we also upgrade existing connection to transaction and run query. Then we close transaction and use connection.

```sh
cargo run --example transaction --features "nightly r2d2_postgres"
```