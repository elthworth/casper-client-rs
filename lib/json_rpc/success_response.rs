use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use super::JsonRpcId;

const JSON_RPC_VERSION: &str = "2.0";

/// A successful response to a JSON-RPC request.
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct SuccessResponse<T> {
    /// A String specifying the version of the JSON-RPC protocol.
    pub jsonrpc: String,
    /// The same identifier as provided in the corresponding request's `id` field.
    pub id: JsonRpcId,
    /// The returned result.
    pub result: T,
}

impl<T> SuccessResponse<T> {
    /// Constructs a new `SuccessResponse`.
    pub fn new(id: JsonRpcId, result: T) -> Self {
        SuccessResponse {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            result,
        }
    }
}

impl<T: Serialize> Display for SuccessResponse<T> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            serde_json::to_string(self).map_err(|_| fmt::Error)?
        )
    }
}
