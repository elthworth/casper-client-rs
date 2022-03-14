use serde::{Deserialize, Serialize};

use casper_types::{ProtocolVersion, Transfer};

use crate::{rpcs::common::BlockIdentifier, types::BlockHash};

pub(crate) const GET_BLOCK_TRANSFERS_METHOD: &str = "chain_get_block_transfers";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetBlockTransfersParams {
    block_identifier: BlockIdentifier,
}

impl GetBlockTransfersParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetBlockTransfersParams { block_identifier }
    }
}

/// The `result` field of a successful JSON-RPC response to a `chain_get_block_transfers` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetBlockTransfersResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block hash, if found.
    pub block_hash: Option<BlockHash>,
    /// The block's transfers, if found.
    pub transfers: Option<Vec<Transfer>>,
}
