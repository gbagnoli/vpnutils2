#[macro_use]
extern crate diesel;
extern crate clap;

pub mod models;
pub mod schema;

use crate::diesel::Connection;
use diesel::sqlite::SqliteConnection;
use clap::App;

pub fn establish_connection(database_url: std::string::String) -> SqliteConnection {
    println!("Connecting to sqlite database at {}", database_url);
    let conn = SqliteConnection::establish(&database_url).expect("cannot open sqlite database");
    conn.execute("PRAGMA foreign_keys = ON").expect("Error trying to enable foreign keys");
    conn
}

pub fn parse_args() -> clap::ArgMatches {
    let config_yaml = clap::load_yaml!("args.yaml");
    App::from(config_yaml).version(clap::crate_version!()).get_matches()
}
