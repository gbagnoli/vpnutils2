use crate::schema::{allowed_ips, networks, peer_statuses, peers, preshared_keys, vpns};

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "networks"]
#[primary_key("name")]
pub struct Network {
    pub name: String,
    pub address_v4: String,
    pub address_v6: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "vpns"]
#[primary_key("name")]
#[belongs_to(Network, foreign_key = "network_name")]
pub struct Vpn {
    pub name: String,
    pub network_name: String,
    pub index_in_network: i32,
    pub address_v4: String,
    pub address_v6: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[table_name = "peers"]
#[primary_key("name")]
#[belongs_to(Vpn, foreign_key = "vpn_name")]
pub struct Peer {
    pub vpn_name: String,
    pub name: String,
    pub index_in_vpn: i32,
    #[column_name = "privkey"]
    pub private_key: String,
    #[column_name = "pubkey"]
    pub public_key: String,
    pub address_v4: String,
    pub address_v6: String,
    pub endpoint: String,
    pub dns: String,
    pub status: String,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "peer_statuses"]
#[primary_key("status")]
pub struct PeerStatus {
    pub status: String,
}

// cannot use Associations here - it doesn't support composite fkeys
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "allowed_ips"]
#[primary_key("peer_vpn", "peer_name", "address")]
pub struct AllowedIp {
    pub peer_vpn: String,
    pub peer_name: String,
    pub address: String,
}

// cannot use Associations here - it doesn't support composite fkeys
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "preshared_keys"]
#[primary_key("vpn", "peer1", "peer2")]
pub struct PresharedKey {
    pub vpn: String,
    pub peer1: String,
    pub peer2: String,
    pub key: String,
}
