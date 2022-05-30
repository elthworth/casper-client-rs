use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

pub(crate) use crate::rpcs::v1_4_5::get_deploy::{GetDeployParams, GET_DEPLOY_METHOD};
use crate::types::{BlockHashAndHeight, Deploy, ExecutionResult};

/// The `result` field of a successful JSON-RPC response to an `info_get_deploy` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetDeployResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy.
    pub deploy: Deploy,
    /// The map of block hash to execution result.
    pub execution_results: Vec<ExecutionResult>,
    /// The hash and height of the block in which this deploy was executed,
    /// only provided if the full execution results are not know on this node.
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub block_hash_and_height: Option<BlockHashAndHeight>,
}
