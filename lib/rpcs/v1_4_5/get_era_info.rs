use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{EraId, ProtocolVersion};

use crate::{
    rpcs::common::BlockIdentifier,
    types::{BlockHash, StoredValue},
};

pub(crate) const GET_ERA_INFO_METHOD: &str = "chain_get_era_info_by_switch_block";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetEraInfoParams {
    block_identifier: BlockIdentifier,
}

impl GetEraInfoParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetEraInfoParams { block_identifier }
    }
}

/// The summary of an era.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EraSummary {
    /// The block hash of the switch block.
    pub block_hash: BlockHash,
    /// The era id.
    pub era_id: EraId,
    /// The [`StoredValue::EraInfo`] containing era information.
    pub stored_value: StoredValue,
    /// The hash of the state root.
    pub state_root_hash: Digest,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}

/// The `result` field of a successful JSON-RPC response to a `chain_get_era_info_by_switch_block`
/// request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetEraInfoResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The era summary, if found.
    pub era_summary: Option<EraSummary>,
}
