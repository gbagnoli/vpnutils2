use age::secrecy::Secret;
use anyhow::{Context, Result};
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use std::fs::File;
use std::io::{Read, Write};

embed_migrations!();

pub struct Database {
    directory: tempfile::TempDir,
    database_path: String,
    backup_path: String,
    source_path: String,
    password: String,
}

fn path_to_string(path: &std::path::PathBuf) -> Result<String> {
    match path.as_path().to_str().map(|s| s.to_string()) {
        None => Err(anyhow::anyhow!("Cannot convert path UTF-8")),
        Some(x) => Ok(x),
    }
}

impl Database {
    pub fn create(source: std::path::PathBuf, password: String) -> Result<Self> {
        let db = Self::new(source, password)?;
        // need to create a new file with diesel setup
        // then move it to the temp directory, and save it
        let conn = SqliteConnection::establish(&db.database_path)?;
        println!("Running migrations on new db at {}", db.database_path);
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
        let mut input = File::open(&self.backup_path)
            .with_context(|| format!("Trying to open the backup file {}", self.backup_path))?;
        let output = File::create(&self.source_path)
            .with_context(|| format!("Trying to open the source file {}", self.source_path))?;

        println!("Encrypting database to {}", self.source_path);
        let mut buffer = vec![];
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(self.password.to_owned()));
        let mut writer = encryptor
            .wrap_output(output)
            .with_context(|| format!("Setting up the encryptor for {}", self.source_path))?;
        input.read_to_end(&mut buffer)?;
        writer.write(&buffer[..])?;
        writer.finish()?;
        std::fs::remove_file(&self.backup_path)?;
        Ok(())
    }

    fn decrypt(&self) -> Result<()> {
        let input = File::open(&self.source_path)
            .with_context(|| format!("Trying to open the source file {}", self.source_path))?;
        let mut output = File::create(&self.database_path).with_context(|| {
            format!(
                "Trying to create the destination file {}",
                self.database_path
            )
        })?;
        println!("Decrypting database from {}", self.source_path);
        let decryptor = match age::Decryptor::new(&input).with_context(|| {
            format!(
                "While setting up decryptor for file at {}",
                self.source_path
            )
        })? {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };
        let mut reader = decryptor.decrypt(&Secret::new(self.password.to_owned()), None)?;
        let mut buffer = vec![];
        reader
            .read_to_end(&mut buffer)
            .with_context(|| format!("Cannot read source file at {}", self.source_path))?;
        output.write(&buffer[..])?;
        Ok(())
    }

    fn backup(&self) -> Result<()> {
        let conn = self.connect()?;
        // this is an alternative to the backup API https://www.sqlite.org/lang_vacuum.html#vacuuminto
        let sql = format!("VACUUM main INTO '{}'", self.backup_path);
        println!("Saving database to {}", self.backup_path);
        conn.execute(&sql)
            .with_context(|| format!("Error while calling VACUUM: {}", sql))?;
        Ok(())
    }
}
