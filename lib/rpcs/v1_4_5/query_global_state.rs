use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{Key, ProtocolVersion};

use crate::types::{BlockHash, BlockHeader, StoredValue};
#[cfg(doc)]
use crate::BlockIdentifier;

pub(crate) const QUERY_GLOBAL_STATE_METHOD: &str = "query_global_state";

/// Identifier for possible ways to query global state.
///
/// Soon to be deprecated in favour of [`BlockIdentifier`].
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum GlobalStateIdentifier {
    /// Query using a block hash.
    BlockHash(BlockHash),
    /// Query using the state root hash.
    StateRootHash(Digest),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct QueryGlobalStateParams {
    state_identifier: GlobalStateIdentifier,
    // `Key` as formatted string.
    key: String,
    path: Vec<String>,
}

impl QueryGlobalStateParams {
    pub(crate) fn new(
        state_identifier: GlobalStateIdentifier,
        key: Key,
        path: Vec<String>,
    ) -> Self {
        QueryGlobalStateParams {
            state_identifier,
            key: key.to_formatted_string(),
            path,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `query_global_state` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueryGlobalStateResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block header if the query was made using a block hash.
    pub block_header: Option<BlockHeader>,
    /// The stored value.
    pub stored_value: StoredValue,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
