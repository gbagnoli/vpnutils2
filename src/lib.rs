#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod args;
mod connection;

#[allow(clippy::unused_unit)]
pub mod models;
pub mod schema;

pub use args::Cli;
pub use connection::{Database, DatabaseError};
