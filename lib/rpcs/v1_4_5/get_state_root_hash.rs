use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::ProtocolVersion;

use crate::rpcs::common::BlockIdentifier;

pub(crate) const GET_STATE_ROOT_HASH_METHOD: &str = "chain_get_state_root_hash";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetStateRootHashParams {
    block_identifier: BlockIdentifier,
}

impl GetStateRootHashParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetStateRootHashParams { block_identifier }
    }
}

/// The `result` field of a successful JSON-RPC response to a `chain_get_state_root_hash` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetStateRootHashResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// Hex-encoded hash of the state root, if found.
    pub state_root_hash: Option<Digest>,
}
