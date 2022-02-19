use age::secrecy::Secret;
use anyhow::Context;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use std::fs::File;
use std::io::{Read, Write};
use thiserror::Error;

embed_migrations!();

pub struct Database {
    // not used yet
    #[allow(dead_code)]
    directory: tempfile::TempDir,
    database_path: String,
    backup_path: String,
    source_path: String,
    password: String,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Cannot convert UTF-8 path")]
    CannotConvertPath(),
    #[error("Cannot open database file at `{path}`")]
    OpenError {
        source: std::io::Error,
        path: String,
    },
    #[error("Cannot create database file at `{path}`")]
    CreateError {
        source: std::io::Error,
        path: String,
    },
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Cannot connect to database")]
    ConnectionError(#[from] diesel::ConnectionError),
    #[error("Cannot run migrations on database")]
    MigrationsError(#[from] diesel_migrations::RunMigrationsError),
    #[error("Error while encrypting file")]
    EncryptError(#[from] age::EncryptError),
    #[error("Decrypt error: corrupt file or wrong password")]
    DecryptError(#[from] age::DecryptError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
type Result<T> = std::result::Result<T, DatabaseError>;

fn path_to_string(path: &std::path::Path) -> Result<String> {
    Ok(path
        .to_str()
        .map(|s| s.to_string())
        .ok_or(DatabaseError::CannotConvertPath())?)
}

impl Database {
    fn new(source: std::path::PathBuf, password: String) -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let db_path = path_to_string(&dir.path().join("database.db"))?;
        let backup_path = path_to_string(&dir.path().join("backup.db"))?;
        let source_path = path_to_string(&source)?;
        let db = Database {
            directory: dir,
            database_path: db_path,
            backup_path,
            source_path,
            password,
        };
        Ok(db)
    }

    pub fn create(source: std::path::PathBuf, password: String) -> Result<Self> {
        let db = Self::new(source, password)?;
        // need to create a new file with diesel setup
        // then move it to the temp directory, and save it
        println!("Creating new database...");
        let conn = SqliteConnection::establish(&db.database_path)?;
        println!("Running migrations...");
        embedded_migrations::run(&conn)?;
        db.save()?;
        Ok(db)
    }

    pub fn open(source: std::path::PathBuf, password: String) -> Result<Self> {
        let db = Self::new(source, password)?;
        db.decrypt()?;
        embedded_migrations::run(&db.connect()?)?;
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
        let output =
            File::create(&self.source_path).map_err(|source| DatabaseError::CreateError {
                source,
                path: self.source_path.clone(),
            })?;

        println!("Encrypting database to {}", self.source_path);
        let mut buffer = vec![];
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(self.password.to_owned()));
        let mut writer = encryptor.wrap_output(output)?;
        input.read_to_end(&mut buffer)?;
        writer.write_all(&buffer[..])?;
        writer.finish()?;
        std::fs::remove_file(&self.backup_path)?;
        Ok(())
    }

    fn decrypt(&self) -> Result<()> {
        let input = File::open(&self.source_path).map_err(|source| DatabaseError::OpenError {
            source,
            path: self.source_path.clone(),
        })?;
        let mut output = File::create(&self.database_path)?;
        println!("Decrypting database from {}", self.source_path);
        let decryptor = match age::Decryptor::new(&input)? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };
        let mut reader = decryptor.decrypt(&Secret::new(self.password.to_owned()), None)?;
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        output.write_all(&buffer[..])?;
        Ok(())
    }

    fn backup(&self) -> Result<()> {
        let conn = self.connect()?;
        // this is an alternative to the backup API https://www.sqlite.org/lang_vacuum.html#vacuuminto
        let sql = format!("VACUUM main INTO '{}'", self.backup_path);
        println!("Saving database to temporary location {}", self.backup_path);
        conn.execute(&sql)
            .with_context(|| format!("Error while calling VACUUM: {}", sql))?;
        Ok(())
    }
}
