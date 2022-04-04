use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

pub(crate) const GET_PEERS_METHOD: &str = "info_get_peers";

/// Peer details.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PeerEntry {
    /// Peer's node ID.
    pub node_id: String,
    /// Peer's address.
    pub address: String,
}

/// The `result` field of a successful JSON-RPC response to a `info_get_peers` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetPeersResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The node ID and network address of each connected peer.
    pub peers: Vec<PeerEntry>,
}
