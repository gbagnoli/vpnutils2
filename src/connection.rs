use age::secrecy::Secret;
use anyhow::{Context, Result};
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use std::fs::File;
use std::io::{Read, Write};

pub struct Database {
    directory: tempfile::TempDir,
    database_path: String,
    backup_path: String,
    source: std::path::PathBuf,
    password: String,
}

impl Database {
    pub fn create(source: std::path::PathBuf, password: String) -> Result<Self> {
        // need to create a new file with diesel setup
        // then move it to the temp directory, and save it
        Self::open(source, password)
    }

    pub fn open(source: std::path::PathBuf, password: String) -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let db_path = dir
            .path()
            .join("database.db")
            .into_os_string()
            .into_string()
            .expect("cannot decode database url");

        let backup_path = dir
            .path()
            .join("backup.db")
            .into_os_string()
            .into_string()
            .expect("cannot decode database url");

        let db = Database {
            directory: dir,
            database_path: db_path,
            backup_path,
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

    pub fn save(&self) -> Result<()> {
        self.backup()?;
        self.encrypt()
    }

    fn encrypt(&self) -> Result<()> {
        let mut input = File::open(&self.backup_path)?;
        let output = File::open(&self.source)?;
        let mut buffer = vec![];
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(self.password.to_owned()));
        let mut writer = encryptor.wrap_output(output)?;
        input.read_to_end(&mut buffer)?;
        writer.write(&buffer[..])?;
        writer.finish()?;
        Ok(())
    }

    fn decrypt(&self) -> Result<()> {
        let input = File::open(&self.source)?;
        let mut output = File::open(&self.database_path)?;
        let decryptor = match age::Decryptor::new(&input)? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };
        let mut reader = decryptor.decrypt(&Secret::new(self.password.to_owned()), None)?;
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        output.write(&buffer[..])?;
        Ok(())
    }

    fn backup(&self) -> Result<()> {
        let conn = self.connect()?;
        // this is an alternative to the backup API https://www.sqlite.org/lang_vacuum.html#vacuuminto
        conn.execute(&format!("VACUUM INTO {}", self.backup_path))
            .context("Error while calling VACUUM")?;
        Ok(())
    }
}
