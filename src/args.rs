use clap::{ArgEnum, Parser, Subcommand};

/// Manage wireguard secrets and peers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// encrypted database file
    #[clap(short, long, parse(from_os_str), env = "DATABASE_PATH")]
    pub database_path: std::path::PathBuf,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage networks
    Network {
        #[clap(subcommand)]
        command: Network,
    },
    /// Manage VPNs
    Vpn {
        #[clap(subcommand)]
        command: Vpn,
    },
    /// Manage peers of a single VPN
    Peer {
        #[clap(subcommand)]
        command: Peer,
    },
}

#[derive(Subcommand, Debug)]
pub enum Network {
    /// List all networks
    List {},
    /// Add a network
    Add {
        name: String,
        ipv4: String,
        ipv6: String,
    },
    /// Remove a network
    Remove { name: String },
    /// Update an existing network
    Update {
        name: String,
        #[clap(short, long)]
        new_name: Option<String>,
        #[clap(long)]
        ipv4: Option<String>,
        #[clap(long)]
        ipv6: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Vpn {
    /// List all vpns
    List {
        /// Restrict to a specific network
        network: Option<String>,
    },
    /// Add a new VPN. A new ipv4 (/24) and ipv6 (/16) subnet will be assigned automatically if
    /// not set
    Add {
        /// name of (existing) network
        network: String,
        /// name of the new VPN
        name: String,
        /// new ipv4 subnet to assign
        #[clap(long)]
        ipv4: Option<String>,
        /// new ipv6 subnet to assign
        #[clap(long)]
        ipv6: Option<String>,
    },
    /// Remove a VPN. VPNs can be removed only if they don't have peers
    Remove { name: String },
    /// Update an existing network
    Update {
        name: String,
        #[clap(short, long)]
        new_name: Option<String>,
        #[clap(long)]
        ipv4: Option<String>,
        #[clap(long)]
        ipv6: Option<String>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum PeerStatus {
    Active,
    Disabled,
}

#[derive(Subcommand, Debug)]
pub enum Peer {
    /// List all peers
    List {
        /// name of the vpn
        vpn: String,
    },
    /// Add a new peer in a VPN
    Add {
        /// vpn the new peer is part of
        vpn: String,
        /// new peer name
        name: String,
        /// peer endpoint
        #[clap(short, long)]
        endpoint: Option<String>,
        /// dns for the peer
        #[clap(short, long)]
        dns: Option<String>,
        /// initial status of the peer
        #[clap(short, long, default_value_t = PeerStatus::Active, arg_enum)]
        status: PeerStatus,
        /// set the public key for the peer
        #[clap(short, long)]
        pubkey: Option<String>,
        /// set the private key for the peer
        #[clap(short = 'P', long)]
        privatekey: Option<String>,
        /// new ipv4 to assign
        #[clap(long)]
        ipv4: Option<String>,
        /// new ipv6 to assign
        #[clap(long)]
        ipv6: Option<String>,
    },
    /// remove a peer from a VPN
    Remove {
        /// vpn the new peer is part of
        vpn: String,
        /// new peer name
        name: String,
    },
    Update {
        /// vpn the new peer is part of
        vpn: String,
        /// peer name
        name: String,
        /// new name for the peer
        #[clap(short, long)]
        new_name: Option<String>,
        /// peer endpoint
        #[clap(short, long)]
        endpoint: Option<String>,
        /// dns for the peer
        #[clap(short, long)]
        dns: Option<String>,
        /// status of the peer
        #[clap(short, long, arg_enum)]
        status: Option<PeerStatus>,
        /// set the public key for the peer
        #[clap(short, long)]
        pubkey: Option<String>,
        /// set the private key for the peer
        #[clap(short = 'P', long)]
        privatekey: Option<String>,
        /// new ipv4 to assign
        #[clap(long)]
        ipv4: Option<String>,
        /// new ipv6 to assign
        #[clap(long)]
        ipv6: Option<String>,
    },
}
