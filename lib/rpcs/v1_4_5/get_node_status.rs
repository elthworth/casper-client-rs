use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{EraId, ProtocolVersion, PublicKey};

use super::get_peers::PeerEntry;
use crate::types::{BlockHash, TimeDiff, Timestamp};

pub(crate) const GET_NODE_STATUS_METHOD: &str = "info_get_status";

/// Minimal info of a `Block`.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MinimalBlockInfo {
    /// The hash uniquely identifying the block.
    pub hash: BlockHash,
    /// The creation timestamp of the block.
    pub timestamp: Timestamp,
    /// The ID of the era in which the block belongs.
    pub era_id: EraId,
    /// The height of the block in the blockchain.
    pub height: u64,
    /// The root hash of global state after executing the deploys in the block.
    pub state_root_hash: Digest,
    /// The public key of the validator which proposed this block.
    pub creator: PublicKey,
}

/// The activation point for a given protocol version.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActivationPoint {
    /// The era ID at which the new protocol version becomes active.
    EraId(EraId),
    /// The timestamp at which the genesis protocol version becomes active.
    Genesis(Timestamp),
}

/// Information about the next protocol upgrade.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct NextUpgrade {
    /// The point at which the next upgrade activates.
    pub activation_point: ActivationPoint,
    /// The protocol version of the next upgrade.
    pub protocol_version: ProtocolVersion,
}

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
