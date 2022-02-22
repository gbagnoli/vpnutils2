extern crate dotenv;
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Password};
use vpnutils::{Cli, CommandParser, Commands, Database};

fn run(db: vpnutils::Database) -> Result<()> {
    loop {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("vpnutils")
            .interact()?;
        let mut parsed_input = match shellwords::split(&input) {
            Ok(a) => a,
            Err(e) => {
                println!("Error parsing input: {}", e);
                continue;
            }
        };
        parsed_input.insert(0, String::from("vpnutils"));
        let args = match CommandParser::try_parse_from(parsed_input) {
            Ok(a) => a,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let result = match args.command {
            Commands::Quit => {
                println!("bye");
                break;
            }
            Commands::Save => db.save(),
            _ => {
                println!("Not implemented: {:?}", args.command);
                continue;
            }
        };
        match result {
            Ok(_) => continue,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    // TODO implement password from stdin
    // TODO implement commands on commandline
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()?;
    let db = match Database::open(args.database_path.clone(), password.clone()) {
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
