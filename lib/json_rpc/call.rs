use jsonrpc_lite::{JsonRpc, Params};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::{Error, JsonRpcId, SuccessResponse, Verbosity};

const RPC_API_PATH: &str = "rpc";

/// Struct representing a single JSON-RPC call to the casper node.
#[derive(Debug)]
pub(crate) struct Call {
    rpc_id: JsonRpcId,
    node_address: String,
    verbosity: Verbosity,
}

/// `Call` encapsulates JSON-RPC calls made to the casper node service.
impl Call {
    pub(crate) fn new(rpc_id: JsonRpcId, node_address: &str, verbosity: Verbosity) -> Self {
        Self {
            rpc_id,
            node_address: node_address.trim_end_matches('/').to_string(),
            verbosity,
        }
    }

    pub(crate) async fn send_request<P: Serialize, R: DeserializeOwned>(
        self,
        method: &'static str,
        maybe_params: Option<P>,
    ) -> Result<SuccessResponse<R>, Error> {
        let url = if self.node_address.ends_with(RPC_API_PATH) {
            self.node_address
        } else {
            format!("{}/{}", self.node_address, RPC_API_PATH)
        };

        let params = match maybe_params {
            Some(params) => Params::Map(
                json!(params)
                    .as_object()
                    .unwrap_or_else(|| panic!("should be a JSON Map"))
                    .clone(),
            ),
            None => Params::None(()),
        };

        let rpc_request = JsonRpc::request_with_params(&self.rpc_id, method, params);

        crate::json_pretty_print(&rpc_request, self.verbosity)?;

        let client = Client::new();
        let http_response = client
            .post(&url)
            .json(&rpc_request)
            .send()
            .await
            .map_err(|error| Error::FailedToGetResponse {
                rpc_id: self.rpc_id.clone(),
                rpc_method: method,
                error,
            })?;

        if let Err(error) = http_response.error_for_status_ref() {
            return Err(Error::ResponseIsHttpError {
                rpc_id: self.rpc_id.clone(),
                rpc_method: method,
                error,
            });
        }

        let rpc_response: JsonRpc =
            http_response
                .json()
                .await
                .map_err(|error| Error::FailedToParseResponse {
                    rpc_id: self.rpc_id.clone(),
                    rpc_method: method,
                    error,
                })?;

        crate::json_pretty_print(&rpc_response, self.verbosity)?;

        let response_kind = match &rpc_response {
            JsonRpc::Request(_) => "Request",
            JsonRpc::Notification(_) => "Notification",
            JsonRpc::Success(_) => "Success",
            JsonRpc::Error(_) => "Error",
        };

        if let Some(json_value) = rpc_response.get_result().cloned() {
            let value =
                serde_json::from_value(json_value).map_err(|err| Error::InvalidRpcResponse {
                    rpc_id: self.rpc_id.clone(),
                    rpc_method: method,
                    response_kind: response_kind.to_string(),
                    response: json!(rpc_response),
                    source: Some(err),
                })?;
            let success_response = SuccessResponse::new(self.rpc_id.clone(), value);
            return Ok(success_response);
        }

        if let Some(error) = rpc_response.get_error().cloned() {
            return Err(Error::ResponseIsRpcError {
                rpc_id: self.rpc_id.clone(),
                rpc_method: method,
                error,
            });
        }

        Err(Error::InvalidRpcResponse {
            rpc_id: self.rpc_id.clone(),
            rpc_method: method,
            response_kind: response_kind.to_string(),
            response: json!(rpc_response),
            source: None,
        })
    }
}
