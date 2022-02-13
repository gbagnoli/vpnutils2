use diesel::sqlite::SqliteConnection;
use diesel::Connection;

pub fn establish_connection(
    database_url: std::path::PathBuf,
    _password: String,
) -> SqliteConnection {
    let db_path_str = database_url.into_os_string().into_string().unwrap();
    println!("Connecting to sqlite database at {}", &db_path_str);
    // TODO: decrypt database
    let conn = SqliteConnection::establish(&db_path_str).expect("cannot open sqlite database");
    conn.execute("PRAGMA foreign_keys = ON")
        .expect("Error trying to enable foreign keys");
    conn
}
