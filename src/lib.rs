#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod args;
mod database;

#[allow(clippy::unused_unit)]
pub mod models;
pub mod schema;

pub use args::{Cli, CommandParser, Commands};
pub use database::{Database, DatabaseError};
