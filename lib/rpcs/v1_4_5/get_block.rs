use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

use crate::{rpcs::common::BlockIdentifier, types::Block};

pub(crate) const GET_BLOCK_METHOD: &str = "chain_get_block";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetBlockParams {
    block_identifier: BlockIdentifier,
}

impl GetBlockParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetBlockParams { block_identifier }
    }
}

/// The `result` field of a successful JSON-RPC response to a `chain_get_block` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetBlockResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block, if found.
    pub block: Option<Block>,
}
