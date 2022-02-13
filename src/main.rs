extern crate dotenv;
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Password};

fn main() {
    dotenv::dotenv().ok();
    let args = vpnutils::Cli::parse();
    // TODO - integrate with clap here
    let _database_env_url = dotenv::var("DATABASE_URL");
    // FIXME: don't panic :-)
    // let database_url = match args.database {
    //     Some(u) => String::from(u),
    //     None => database_env_url.expect("no database url set on commandline or env DATABASE_URL"),
    // };
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Database Password")
        .interact()
        .unwrap();

    vpnutils::establish_connection(args.database, password);
}
