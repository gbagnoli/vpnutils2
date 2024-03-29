#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod args;
mod commands;
mod database;
#[allow(clippy::unused_unit)]
mod models;
mod schema;

pub use args::{Cli, CommandParser};
pub use commands::Commands;
pub use database::{Database, DatabaseError};
