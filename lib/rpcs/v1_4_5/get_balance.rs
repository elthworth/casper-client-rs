use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{ProtocolVersion, URef, U512};

pub(crate) const GET_BALANCE_METHOD: &str = "state_get_balance";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetBalanceParams {
    state_root_hash: Digest,
    purse_uref: URef,
}

impl GetBalanceParams {
    pub(crate) fn new(state_root_hash: Digest, purse_uref: URef) -> Self {
        GetBalanceParams {
            state_root_hash,
            purse_uref,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `state_get_balance` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetBalanceResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The balance value.
    pub balance_value: U512,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
