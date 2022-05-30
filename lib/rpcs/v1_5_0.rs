//! The JSON-RPC request and response types at v1.5.0 of casper-node.

pub(crate) mod get_chainspec;
pub(crate) mod get_deploy;
pub(crate) mod get_node_status;
pub(crate) mod query_balance;

// The following RPCs are all unchanged from v1.4.5, so we just re-export them.

pub(crate) mod get_account {
    pub use crate::rpcs::v1_4_5::get_account::GetAccountResult;
    pub(crate) use crate::rpcs::v1_4_5::get_account::{GetAccountParams, GET_ACCOUNT_METHOD};
}

pub(crate) mod get_auction_info {
    pub use crate::rpcs::v1_4_5::get_auction_info::GetAuctionInfoResult;
    pub(crate) use crate::rpcs::v1_4_5::get_auction_info::{
        GetAuctionInfoParams, GET_AUCTION_INFO_METHOD,
    };
}

pub(crate) mod get_balance {
    pub use crate::rpcs::v1_4_5::get_balance::GetBalanceResult;
    pub(crate) use crate::rpcs::v1_4_5::get_balance::{GetBalanceParams, GET_BALANCE_METHOD};
}

pub(crate) mod get_block {
    pub use crate::rpcs::v1_4_5::get_block::GetBlockResult;
    pub(crate) use crate::rpcs::v1_4_5::get_block::{GetBlockParams, GET_BLOCK_METHOD};
}

pub(crate) mod get_block_transfers {
    pub use crate::rpcs::v1_4_5::get_block_transfers::GetBlockTransfersResult;
    pub(crate) use crate::rpcs::v1_4_5::get_block_transfers::{
        GetBlockTransfersParams, GET_BLOCK_TRANSFERS_METHOD,
    };
}

pub(crate) mod get_dictionary_item {
    pub use crate::rpcs::v1_4_5::get_dictionary_item::{
        DictionaryItemIdentifier, GetDictionaryItemResult,
    };
    pub(crate) use crate::rpcs::v1_4_5::get_dictionary_item::{
        GetDictionaryItemParams, GET_DICTIONARY_ITEM_METHOD,
    };
}

pub(crate) mod get_era_info {
    pub use crate::rpcs::v1_4_5::get_era_info::{EraSummary, GetEraInfoResult};
    pub(crate) use crate::rpcs::v1_4_5::get_era_info::{GetEraInfoParams, GET_ERA_INFO_METHOD};
}

pub(crate) mod get_peers {
    pub(crate) use crate::rpcs::v1_4_5::get_peers::GET_PEERS_METHOD;
    pub use crate::rpcs::v1_4_5::get_peers::{GetPeersResult, PeerEntry};
}

pub(crate) mod get_state_root_hash {
    pub use crate::rpcs::v1_4_5::get_state_root_hash::GetStateRootHashResult;
    pub(crate) use crate::rpcs::v1_4_5::get_state_root_hash::{
        GetStateRootHashParams, GET_STATE_ROOT_HASH_METHOD,
    };
}

pub(crate) mod get_validator_changes {
    pub(crate) use crate::rpcs::v1_4_5::get_validator_changes::GET_VALIDATOR_CHANGES_METHOD;
    pub use crate::rpcs::v1_4_5::get_validator_changes::{
        GetValidatorChangesResult, ValidatorChange, ValidatorChangeInEra, ValidatorChanges,
    };
}

pub(crate) mod list_rpcs {
    pub(crate) use crate::rpcs::v1_4_5::list_rpcs::LIST_RPCS_METHOD;
    pub use crate::rpcs::v1_4_5::list_rpcs::{
        Components, Example, ExampleParam, ExampleResult, ListRpcsResult, Method,
        OpenRpcContactField, OpenRpcInfoField, OpenRpcLicenseField, OpenRpcSchema,
        OpenRpcServerEntry, ResponseResult, SchemaParam,
    };
}

pub(crate) mod put_deploy {
    pub use crate::rpcs::v1_4_5::put_deploy::PutDeployResult;
    pub(crate) use crate::rpcs::v1_4_5::put_deploy::{PutDeployParams, PUT_DEPLOY_METHOD};
}

pub(crate) mod query_global_state {
    pub use crate::rpcs::v1_4_5::query_global_state::{
        GlobalStateIdentifier, QueryGlobalStateResult,
    };
    pub(crate) use crate::rpcs::v1_4_5::query_global_state::{
        QueryGlobalStateParams, QUERY_GLOBAL_STATE_METHOD,
    };
}
