extern crate dotenv;
use clap::Parser;

fn main() {
    dotenv::dotenv().ok();
    let database_env_url = dotenv::var("DATABASE_URL");
    let args = vpnutils::Cli::parse();
    let database_url = match args.database {
        Some(u) => String::from(u),
        None => database_env_url.expect("no database url set on commandline or env DATABASE_URL"),
    };
    vpnutils::establish_connection(database_url);
}
