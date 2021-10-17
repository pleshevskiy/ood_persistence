#![deny(clippy::all)]
#![cfg_attr(feature = "nightly", feature(generic_associated_types))]

#[cfg(feature = "async")]
#[macro_use]
extern crate async_trait;

#[cfg(feature = "async")]
pub mod asyn;

#[cfg(feature = "sync")]
pub mod syn;

#[cfg(feature = "bb8")]
pub use bb8;

#[cfg(feature = "bb8_postgres")]
pub mod bb8_postgres;

#[cfg(feature = "r2d2")]
pub use r2d2;

#[cfg(feature = "r2d2_postgres")]
pub mod r2d2_postgres;

pub mod error;
