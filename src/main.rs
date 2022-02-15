extern crate dotenv;
use anyhow::Result;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Password};

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = vpnutils::Cli::parse();
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()?;

    // let db = vpnutils::Database::create(args.database_path, password)?;
    match vpnutils::Database::open(args.database_path.clone(), password.clone()) {
        Ok(db) => {
            println!("Connecting to database");
            db.connect()?;
            db.save()
        }
        Err(_) => {
            vpnutils::Database::create(args.database_path, password)?;
            Ok(())
        }
    }
}
