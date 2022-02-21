extern crate dotenv;
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Password};

fn run(_db: vpnutils::Database) -> Result<()> {
    loop {
        let command: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Command")
            .interact()?;
        // TODO parse from string for CommandParser
        if command == "quit" {
            println!("Bye!");
            break;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = vpnutils::Cli::parse();
    // TODO implement password from stdin
    // TODO implement commands on commandline
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()?;
    let db = match vpnutils::Database::open(args.database_path.clone(), password.clone()) {
        Ok(db) => {
            println!("Connecting to database");
            db.connect()?;
            db
        }
        Err(e) => match e {
            vpnutils::DatabaseError::OpenError { source: _, path } => {
                println!("Database {} does not exist - creating", path);
                vpnutils::Database::create(args.database_path, password)
                    .context("cannot create database")?
            }
            vpnutils::DatabaseError::DecryptError(_) => {
                return Err(anyhow::anyhow!("Invalid password, cannot decrypt"));
            }
            _ => return Err(anyhow::anyhow!("Unknown error")),
        },
    };
    run(db)
}
