# OOD Persistence

[![Crates.io](https://img.shields.io/crates/v/ood_persistence)](https://crates.io/crates/ood_persistence)
[![Documentation](https://docs.rs/ood_persistence/badge.svg)](https://docs.rs/ood_persistence)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Asynchronous and synchronous interfaces and persistence implementations for your OOD architecture

## Installation

Add `ood_persistence = { version = "0", features = ["<IMPLEMENTATION_NAME>"] }` as a dependency in `Cargo.toml`.

NOTE: change `<IMPLEMENTATION_NAME>` to feature name from available list. See `Cargo.toml` for more information.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
ood_persistence = { version = "0", features = ["bb8_postgres"] }
```

In stable rust channel you can use only connection interface, but if you use nightly channel, add an additional
"nightly" feature to your `Cargo.toml` and you can use transactions as well.

## Usage

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create pool to our database
    let pool = create_postgres_pool();
    // initialize persistence
    let persistence = ood_persistence::r2d2_postgres::new(&pool);
    // get connection to database
    let mut conn = persistence.get_connection()?;
    // we can query something
    let res: i32 = conn.inner().query_one("select 1", &[])?.get(0);
    assert_eq!(res, 1);

    Ok(())
}
```

See examples directory for more information.


## Contributors

[pleshevskiy](https://github.com/pleshevskiy) (Dmitriy Pleshevskiy) – creator, maintainer.
