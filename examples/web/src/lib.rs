#![deny(clippy::all)]

#[macro_use]
extern crate postgres_types;
#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

pub mod config;
pub mod error;

mod app;
pub mod db;
pub mod rest;
