use clap::Parser;

/// Manage wireguard secrets and peers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// encrypted database file
    #[clap(short, long, parse(from_os_str))]
    pub database: std::path::PathBuf,
}
