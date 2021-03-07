#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use crate::diesel::Connection;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL env var must be set");
    SqliteConnection::establish(&database_url).expect("cannot open sqlite database")
}
