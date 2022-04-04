use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

use crate::{rpcs::common::BlockIdentifier, types::AuctionState};

pub(crate) const GET_AUCTION_INFO_METHOD: &str = "state_get_auction_info";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetAuctionInfoParams {
    block_identifier: BlockIdentifier,
}

impl GetAuctionInfoParams {
    pub(crate) fn new(block_identifier: BlockIdentifier) -> Self {
        GetAuctionInfoParams { block_identifier }
    }
}

/// The `result` field of a successful JSON-RPC response to a `state_get_auction_info` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetAuctionInfoResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The auction state.
    pub auction_state: AuctionState,
}
