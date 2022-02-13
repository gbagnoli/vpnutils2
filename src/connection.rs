use anyhow::{Context, Result};
use diesel::sqlite::SqliteConnection;
use diesel::Connection;

pub struct Database {
    directory: tempfile::TempDir,
    database_path: String,
    source: std::path::PathBuf,
    password: String,
}

impl Database {
    pub fn new(source: std::path::PathBuf, password: String) -> Result<Database> {
        let dir = tempfile::tempdir()?;
        let db_path = dir
            .path()
            .join("database.db")
            .into_os_string()
            .into_string()
            .expect("cannot decode database url");

        let db = Database {
            directory: dir,
            database_path: db_path,
            source,
            password,
        };
        db.decrypt()?;
        Ok(db)
    }

    pub fn connect(&self) -> Result<SqliteConnection> {
        let conn = SqliteConnection::establish(&self.database_path)
            .context("Cannot open sqlite database")?;
        conn.execute("PRAGMA foreign_keys = ON")
            .context("Error trying to enable foreign keys")?;
        Ok(conn)
    }

    fn decrypt(&self) -> Result<()> {
        Ok(())
    }

    fn encrypt(&self) -> Result<()> {
        Ok(())
    }
}
