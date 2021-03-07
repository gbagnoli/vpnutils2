use diesel::table;

table! {
    allowed_ips (peer_vpn, peer_name, address) {
        peer_vpn -> Text,
        peer_name -> Text,
        address -> Text,
    }
}

table! {
    networks (name) {
        name -> Text,
        address_v4 -> Text,
        address_v6 -> Text,
    }
}

table! {
    peer_statuses (status) {
        status -> Text,
    }
}

table! {
    peers (vpn_name, name) {
        vpn_name -> Text,
        name -> Text,
        index_in_vpn -> Nullable<Integer>,
        privkey -> Text,
        pubkey -> Text,
        address_v4 -> Text,
        address_v6 -> Text,
        endpoint -> Nullable<Text>,
        dns -> Nullable<Text>,
        status -> Text,
    }
}

table! {
    preshared_keys (vpn, peer1, peer2) {
        vpn -> Text,
        peer1 -> Text,
        peer2 -> Text,
        key -> Text,
    }
}

table! {
    vpns (name) {
        name -> Text,
        network_name -> Text,
        index_in_network -> Nullable<Integer>,
        address_v4 -> Text,
        address_v6 -> Text,
    }
}

joinable!(peers -> peer_statuses (status));
joinable!(peers -> vpns (vpn_name));
joinable!(vpns -> networks (network_name));

allow_tables_to_appear_in_same_query!(
    allowed_ips,
    networks,
    peer_statuses,
    peers,
    preshared_keys,
    vpns,
);
