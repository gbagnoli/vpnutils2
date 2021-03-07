PRAGMA foreign_keys=ON;
CREATE TABLE `networks` (
  `name` TEXT PRIMARY KEY,
  `address_v4` TEXT UNIQUE NOT NULL,
  `address_v6` TEXT UNIQUE NOT NULL
) WITHOUT ROWID;

CREATE TABLE `vpns` (
  `name` TEXT PRIMARY KEY,
  `network_name` TEXT NOT NULL,
  `index_in_network` INT,
  `address_v4` TEXT UNIQUE NOT NULL,
  `address_v6` TEXT UNIQUE NOT NULL,
  FOREIGN KEY (`network_name`) REFERENCES `networks` (`name`) ON UPDATE CASCADE ON DELETE RESTRICT
) WITHOUT ROWID;
CREATE INDEX `vpns_network_names_idx` ON `vpns`(`network_name`);
CREATE UNIQUE INDEX `vpn_index_in_network` ON `vpns`(`network_name`, `index_in_network`);
/* when creating a vpn, assign its index in the network */
CREATE TRIGGER assign_a_vpn_the_next_index AFTER INSERT ON `vpns`
BEGIN
  UPDATE vpns SET index_in_network = (SELECT MAX(index_in_network)+1 FROM `vpns` GROUP BY network_name HAVING network_name = new.network_name)
    WHERE name = new.name;
END;

CREATE TABLE `peer_statuses` (
  `status` TEXT PRIMARY KEY NOT NULL
) WITHOUT ROWID;
INSERT INTO `peer_statuses` VALUES ("active"), ("disabled");

CREATE TABLE `peers` (
  `vpn_name` TEXT NOT NULL,
  `name` TEXT NOT NULL,
  `index_in_vpn` INT,
  `privkey` TEXT NOT NULL,
  `pubkey` TEXT NOT NULL,
  `address_v4` TEXT NOT NULL,
  `address_v6` TEXT NOT NULL,
  `endpoint` TEXT,
  `dns` TEXT,
  `status` TEXT NOT NULL,
  PRIMARY KEY (`vpn_name`, `name`)
  FOREIGN KEY (`vpn_name`) REFERENCES `vpns` (`name`) ON UPDATE CASCADE ON DELETE RESTRICT
  FOREIGN KEY (`status`) REFERENCES `peer_statuses`(`status`) ON UPDATE CASCADE ON DELETE RESTRICT
) WITHOUT ROWID;
CREATE INDEX `peers_vpn_names_idx` ON `peers`(`vpn_name`);
CREATE UNIQUE INDEX `peers_index_in_vpn` ON `peers`(`vpn_name`, `index_in_vpn`);
CREATE INDEX `peers_statuses_idx` ON `peers`(`status`);

/* when creating a peer, assign its id */
CREATE TRIGGER assign_a_peer_the_next_index AFTER INSERT ON `peers`
BEGIN
  UPDATE peers SET index_in_vpn = (SELECT MAX(index_in_vpn)+1 FROM `peers` GROUP BY `vpn_name` HAVING vpn_name = new.vpn_name)
    WHERE vpn_name = new.vpn_name AND name = new.name;
END;

/* Copy the IPv4 and IPv6 addresses of a peer into its allowed addresses */
CREATE TRIGGER after_insert_on_peers_add_allowed_ips AFTER INSERT ON `peers`
BEGIN
  INSERT INTO allowed_ips VALUES(new.vpn_name, new.name, new.address_v4);
  INSERT INTO allowed_ips VALUES(new.vpn_name, new.name, new.address_v6);
END;

/* Update IPv4/IPv6 allowed_ips of a peer when they are updated in the peer table */
CREATE TRIGGER after_update_on_peers_update_allowed_ips AFTER UPDATE ON `peers`
  WHEN new.address_v4 <> old.address_v4
    OR new.address_v6 <> old.address_v6
BEGIN
  UPDATE allowed_ips
    SET address = new.address_v4
    WHERE peer_vpn = new.vpn_name
      AND peer_name = new.name
      AND address = old.address_v4;
  UPDATE allowed_ips
    SET address = new.address_v6
    WHERE peer_vpn = new.vpn_name
      AND peer_name = new.name
      AND address = old.address_v6;
END;

CREATE TABLE `allowed_ips` (
  `peer_vpn` TEXT NOT NULL,
  `peer_name` TEXT NOT NULL,
  `address` TEXT NOT NULL,
  PRIMARY KEY (`peer_vpn`, `peer_name`, `address`)
  FOREIGN KEY (`peer_vpn`, `peer_name`) REFERENCES `peers` (`vpn_name`, `name`) ON UPDATE CASCADE ON DELETE CASCADE
) WITHOUT ROWID;
CREATE INDEX `allowed_ips_peers_idx` ON `allowed_ips`(`peer_vpn`, `peer_name`);

CREATE TABLE `preshared_keys` (
  `vpn` TEXT NOT NULL,
  `peer1` TEXT NOT NULL,
  `peer2` TEXT NOT NULL,
  `key` TEXT NOT NULL,
  PRIMARY KEY (`vpn`, `peer1`, `peer2`)
  FOREIGN KEY (`vpn`, `peer1`) REFERENCES `peers` (`vpn_name`, `name`) ON UPDATE CASCADE ON DELETE CASCADE
  FOREIGN KEY (`vpn`, `peer2`) REFERENCES `peers` (`vpn_name`, `name`) ON UPDATE CASCADE ON DELETE CASCADE
) WITHOUT ROWID;
CREATE INDEX `preshared_keys_vpns_idx` ON `preshared_keys`(`vpn`);
CREATE INDEX `preshared_keys_peers1_idx` ON `preshared_keys`(`peer1`);
CREATE INDEX `preshared_keys_peers2_idx` ON `preshared_keys`(`peer2`);

/* preshared_keys are between two peers, so avoid having peer1, peer2 and peer2, peer1 */
CREATE TRIGGER before_insert_check_if_pair_is_present_for_preshared_keys BEFORE INSERT ON `preshared_keys`
BEGIN
  SELECT RAISE(FAIL, "peers pair already have a preshared_key")
  FROM preshared_keys
  WHERE vpn = new.vpn AND peer2 = new.peer1 AND peer1 = new.peer2;
END;
