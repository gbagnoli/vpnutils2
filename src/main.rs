extern crate dotenv;

fn main() {
    dotenv::dotenv().ok();
    let database_env_url = dotenv::var("DATABASE_URL");
    let matches = vpnutils::parse_args();
    let database_url = match matches.value_of("database") {
        Some(u) => String::from(u),
        None => database_env_url.expect("no database url set on commandline or env DATABASE_URL"),
    };
    vpnutils::establish_connection(database_url);
}
