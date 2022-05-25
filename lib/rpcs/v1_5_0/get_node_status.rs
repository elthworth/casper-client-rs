pub(crate) use crate::rpcs::v1_4_5::get_node_status::GET_NODE_STATUS_METHOD;
pub use crate::rpcs::v1_4_5::get_node_status::{ActivationPoint, MinimalBlockInfo, NextUpgrade};

use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{ProtocolVersion, PublicKey};

use super::get_peers::PeerEntry;
use crate::types::TimeDiff;

/// The various possible states of operation for the node.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum NodeState {
    /// The node is fast-syncing.
    FastSyncing,
    /// The node is syncing to genesis.
    SyncingToGenesis,
    /// The node is participating.
    Participating,
}

/// The `result` field of a successful JSON-RPC response to a `info_get_status` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetNodeStatusResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The chainspec name.
    pub chainspec_name: String,
    /// The state root hash used at the start of the current session.
    #[deprecated(since = "1.5.0")]
    pub starting_state_root_hash: Digest,
    /// The node ID and network address of each connected peer.
    pub peers: Vec<PeerEntry>,
    /// The minimal info of the last block from the linear chain.
    pub last_added_block_info: Option<MinimalBlockInfo>,
    /// The node's public signing key.
    pub our_public_signing_key: Option<PublicKey>,
    /// The next round length if this node is a validator.
    pub round_length: Option<TimeDiff>,
    /// Information about the next scheduled upgrade staged for this node.
    pub next_upgrade: Option<NextUpgrade>,
    /// The compiled node version.
    pub build_version: String,
    /// Time that passed since the node has started.
    pub uptime: TimeDiff,
    /// The current state of node.
    pub node_state: NodeState,
}
