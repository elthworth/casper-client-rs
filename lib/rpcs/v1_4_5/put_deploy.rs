use serde::{Deserialize, Serialize};

use casper_types::ProtocolVersion;

use crate::types::{Deploy, DeployHash};

pub(crate) const PUT_DEPLOY_METHOD: &str = "account_put_deploy";

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct PutDeployParams {
    deploy: Deploy,
}

impl PutDeployParams {
    pub(crate) fn new(deploy: Deploy) -> Self {
        PutDeployParams { deploy }
    }
}

/// The `result` field of a successful JSON-RPC response to an `account_put_deploy` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PutDeployResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The deploy hash.
    pub deploy_hash: DeployHash,
}
