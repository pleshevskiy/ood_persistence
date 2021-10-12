# OOD Persistence

Asynchronous and synchronous interfaces and persistence implementations for your OOD architecture

## Installation

Add `ood_persistence = { version = "0", features = ["<IMPLEMENTATION_NAME>"] }` as a dependency in `Cargo.toml`.

NOTE: change `<IMPLEMENTATION_NOTE>` to feature name from available list. See `Cargo.toml` for more information.

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
ood_persistence = { version = "0", features = ["bb8_postgres"] }
```

## Usage

See examples directory.
