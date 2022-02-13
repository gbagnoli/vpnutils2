#[macro_use]
extern crate diesel;

mod args;
pub mod models;
pub mod schema;

pub use args::Cli;

use diesel::sqlite::SqliteConnection;
use diesel::Connection;

pub fn establish_connection(database_url: std::string::String) -> SqliteConnection {
    println!("Connecting to sqlite database at {}", database_url);
    let conn = SqliteConnection::establish(&database_url).expect("cannot open sqlite database");
    conn.execute("PRAGMA foreign_keys = ON")
        .expect("Error trying to enable foreign keys");
    conn
}
