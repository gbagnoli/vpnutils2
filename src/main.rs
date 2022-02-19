extern crate dotenv;
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Password};

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = vpnutils::Cli::parse();
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()?;
    match vpnutils::Database::open(args.database_path.clone(), password.clone()) {
        Ok(db) => {
            println!("Connecting to database");
            db.connect()?;
            db.save().context("Cannot save database")
        }
        Err(e) => match e {
            vpnutils::DatabaseError::OpenError { source: _, path } => {
                println!("Database {} does not exist - creating", path);
                vpnutils::Database::create(args.database_path, password)
                    .context("cannot create database")?;
                Ok(())
            }
            vpnutils::DatabaseError::DecryptError(_) => {
                Err(anyhow::anyhow!("Invalid password, cannot decrypt"))
            }
            _ => Err(anyhow::anyhow!("Unknown error")),
        },
    }
}
