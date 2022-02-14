extern crate dotenv;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Password};

fn main() {
    dotenv::dotenv().ok();
    let args = vpnutils::Cli::parse();
    let password = match Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()
    {
        Err(_e) => {
            println!("Failed to read password");
            std::process::exit(1);
        }
        Ok(p) => p,
    };
    let db = vpnutils::Database::open(args.database_path, password).unwrap();
    println!("Connecting to database");
    db.connect().unwrap();

    db.save().unwrap();
}
