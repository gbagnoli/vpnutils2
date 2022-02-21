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
        /// Name of the new network, must be unique
        name: String,
        /// Ipv4 Network from where to create subnets for vpns
        #[clap(long, short = '4')]
        ipv4: ipnet::Ipv4Net,
        /// Ipv6 Network from where to create subnets for vpns
        #[clap(long, short = '6')]
        ipv6: ipnet::Ipv6Net,
    },
    /// Remove a network
    Remove { name: String },
    /// Update an existing network
    Update {
        /// Existing name of the network to modify
        name: String,
        /// New name to assign to the network
        #[clap(short, long)]
        new_name: Option<String>,
        /// Ipv4 Network from where to create subnets for vpns
        #[clap(long, short = '4')]
        ipv4: Option<ipnet::Ipv4Net>,
        /// Ipv6 Network from where to create subnets for vpns
        #[clap(long, short = '6')]
        ipv6: Option<ipnet::Ipv6Net>,
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
        #[clap(long, short = '4')]
        ipv4: Option<ipnet::Ipv4Net>,
        /// new ipv6 subnet to assign
        #[clap(long, short = '6')]
        ipv6: Option<ipnet::Ipv6Net>,
    },
    /// Remove a VPN. VPNs can be removed only if they don't have peers
    Remove { name: String },
    /// Update an existing network
    Update {
        /// name of (existing) vpn
        name: String,
        /// new name to assign to vpn
        #[clap(short, long)]
        new_name: Option<String>,
        /// new ipv4 subnet to assign
        #[clap(long, short = '4')]
        ipv4: Option<ipnet::Ipv4Net>,
        /// new ipv6 subnet to assign
        #[clap(long, short = '6')]
        ipv6: Option<ipnet::Ipv6Net>,
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
        #[clap(long, short = '4')]
        ipv4: Option<std::net::Ipv4Addr>,
        /// new ipv6 to assign
        #[clap(long, short = '6')]
        ipv6: Option<std::net::Ipv6Addr>,
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
        #[clap(long, short = '4')]
        ipv4: Option<std::net::Ipv4Addr>,
        /// new ipv6 to assign
        #[clap(long, short = '6')]
        ipv6: Option<std::net::Ipv6Addr>,
    },
}
