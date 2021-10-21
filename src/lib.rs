//! # OOD Persistence
//!
//! Asynchronous and synchronous interfaces and persistence implementations for your OOD architecture
//!
//! ## Installation
//!
//! Add `ood_persistence = { version = "0", features = ["<IMPLEMENTATION_NAME>"] }` as a dependency in `Cargo.toml`.
//!
//! NOTE: change `<IMPLEMENTATION_NAME>` to feature name from available list. See `Cargo.toml` for more information.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [package]
//! name = "my-crate"
//! version = "0.1.0"
//! authors = ["Me <user@rust-lang.org>"]
//!
//! [dependencies]
//! ood_persistence = { version = "0", features = ["bb8_postgres"] }
//! ```
//!
//! In stable rust channel you can use only connection interface, but if you use nightly channel, add an additional
//! "nightly" feature to your `Cargo.toml` and you can use transactions as well.
//!
//! ## Usage
//!
//! See examples directory.
//!
#![forbid(unsafe_code, non_ascii_idents)]
#![deny(clippy::all)]
#![warn(missing_docs)]
#![cfg_attr(feature = "nightly", feature(generic_associated_types))]

#[cfg(feature = "async")]
#[macro_use]
extern crate async_trait;

/// This module contains interfaces for async persistence.
///
/// **Note:** This mod requires enabling the `async` feature or any feature
/// with async implementation (for example: `bb8_postgres`)
#[cfg(feature = "async")]
pub mod asyn;

/// This module contains interfaces for sync persistence.
///
/// **Note:** This mod requires enabling the `sync` feature or any feature
/// with sync implementation (for example: `r2d2_postgres`)
#[cfg(feature = "sync")]
pub mod syn;

/// You can get the bb8 module if you activate the `bb8` feature or
/// any feature with bb8 implementation like `bb8_postgres`
#[cfg(feature = "bb8")]
pub use bb8;

/// This module contains implementation for async interface of postgres database.
///
/// The implementation uses `bb8` as the pool and `tokio_postgres` as the client.
///
/// **Note:** This mod requires enabling the `bb8_postgres` feature
#[cfg(feature = "bb8_postgres")]
pub mod bb8_postgres;

/// You can get the r2d2 module if you activate the `r2d2` feature or
/// any feature with r2d2 implementation like `r2d2_postgres`
#[cfg(feature = "r2d2")]
pub use r2d2;

/// This module contains implementation for sync interface of postgres database.
///
/// The implementation uses `r2d2` as the pool and `postgres` as the client.
///
/// **Note:** This mod requires enabling the `r2d2_postgres` feature.
#[cfg(feature = "r2d2_postgres")]
pub mod r2d2_postgres;

/// This module contains implementation for sync interface of sqlite database.
///
/// The implementation uses `r2d2` as the pool and `rusqlite` as the client.
///
/// **Note:** This mod requires enabling the `r2d2_sqlite` feature.
#[cfg(feature = "r2d2_sqlite")]
pub mod r2d2_sqlite;

/// This module contains implementations for errors and result, that this
/// crate uses
pub mod error;
