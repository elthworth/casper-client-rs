use std::{io, path::PathBuf};

use thiserror::Error;

use casper_types::{bytesrepr::Error as ToBytesError, Key};
#[cfg(doc)]
use casper_types::{CLValue, URef};

#[cfg(doc)]
use crate::types::{Deploy, DeployBuilder, TimeDiff, Timestamp};
use crate::{validation::ValidateResponseError, CryptoError, JsonRpcId};

/// Errors that may be returned by `casper_client` functions.
#[derive(Error, Debug)]
pub enum Error {
    /// [`Deploy`] size too large.
    #[error("deploy size of {actual_deploy_size} bytes exceeds limit of {max_deploy_size}")]
    DeploySizeTooLarge {
        /// The maximum permitted serialized deploy size, in bytes.
        max_deploy_size: u32,
        /// The serialized size of the deploy provided, in bytes.
        actual_deploy_size: usize,
    },

    /// Failed to build [`Deploy`] due to missing payment code.
    ///
    /// Call [`DeployBuilder::with_standard_payment`] or [`DeployBuilder::with_payment`] before
    /// calling [`DeployBuilder::build`].
    #[error("deploy requires payment code - use `with_payment` or `with_standard_payment`")]
    DeployMissingPaymentCode,

    /// Invalid [`Key`] variant.
    #[error("expected {} but got {}", .expected_variant, .actual)]
    InvalidKeyVariant {
        /// The expected variant.
        expected_variant: String,
        /// The actual key provided.
        actual: Key,
    },

    /// Failed to get a response from the node.
    #[error("failed to get response for rpc-id {rpc_id} {rpc_method}: {error}")]
    FailedToGetResponse {
        /// The JSON-RPC ID.
        rpc_id: JsonRpcId,
        /// The JSON-RPC request method.
        rpc_method: &'static str,
        /// The reported error.
        error: reqwest::Error,
    },

    /// JSON-RPC error returned from the node.
    #[error("response for rpc-id {rpc_id} {rpc_method} is http error: {error}")]
    ResponseIsHttpError {
        /// The JSON-RPC ID.
        rpc_id: JsonRpcId,
        /// The JSON-RPC request method.
        rpc_method: &'static str,
        /// The reported error.
        error: reqwest::Error,
    },

    /// Failed to parse the response.
    #[error("failed to parse response for rpc-id {rpc_id} {rpc_method}: {error}")]
    FailedToParseResponse {
        /// The JSON-RPC ID.
        rpc_id: JsonRpcId,
        /// The JSON-RPC request method.
        rpc_method: &'static str,
        /// The reported error.
        error: reqwest::Error,
    },

    /// JSON-RPC error returned from the node.
    #[error("response for rpc-id {rpc_id} {rpc_method} is json-rpc error: {error}")]
    ResponseIsRpcError {
        /// The JSON-RPC ID.
        rpc_id: JsonRpcId,
        /// The JSON-RPC request method.
        rpc_method: &'static str,
        /// The reported error.
        error: jsonrpc_lite::Error,
    },

    /// Invalid response returned from the node.
    #[error(
        "response {response_kind} for rpc-id {rpc_id} {rpc_method} is not valid because {source:?}: {response}"
    )]
    InvalidRpcResponse {
        /// The JSON-RPC ID.
        rpc_id: JsonRpcId,
        /// The JSON-RPC request method.
        rpc_method: &'static str,
        /// What kind of Json response was received.
        response_kind: &'static str,
        /// The JSON response.
        response: serde_json::Value,
        /// If available, the original error from Serde.
        source: Option<serde_json::Error>,
    },

    /// Failed to encode to JSON.
    #[error("failed to encode to json: {context}: {error}")]
    FailedToEncodeToJson {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// Underlying encoding error.
        error: serde_json::Error,
    },

    /// Failed to decode from JSON.
    #[error("failed to decode from json: {context}: {error}")]
    FailedToDecodeFromJson {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// Underlying decoding error.
        error: serde_json::Error,
    },

    /// Failed to create new file because it already exists.
    #[error("file at {} already exists", .0.display())]
    FileAlreadyExists(PathBuf),

    /// Empty path provided as output dir for keygen.
    #[error("empty path provided as output dir for keygen")]
    EmptyKeygenPath,

    /// Unsupported keygen algorithm.
    #[error("unsupported keygen algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// Context-adding wrapper for `std::io::Error`.
    #[error("input/output error: {context}: {error}")]
    IoError {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: String,
        /// std::io::Error raised during the operation in question.
        error: io::Error,
    },

    /// Failed to serialize to bytes.
    #[error("serialization error: {0}")]
    ToBytesError(ToBytesError),

    /// Cryptographic error.
    #[error("cryptographic error: {context}: {error}")]
    CryptoError {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// Underlying crypto error.
        error: CryptoError,
    },

    /// Failed to validate response.
    #[error("invalid response: {0}")]
    ResponseFailedValidation(#[from] ValidateResponseError),
}

impl From<ToBytesError> for Error {
    fn from(error: ToBytesError) -> Self {
        Error::ToBytesError(error)
    }
}
