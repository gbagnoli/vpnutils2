use clap::Parser;

/// Manage wireguard secrets and peers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// sqlite database file
    #[clap(short, long)]
    pub database: Option<String>,
}
