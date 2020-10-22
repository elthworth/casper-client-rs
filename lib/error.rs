use std::{num::ParseIntError, path::PathBuf};

use jsonrpc_lite::JsonRpc;
use thiserror::Error;

use casper_node::crypto::Error as CryptoError;
use casper_types::{bytesrepr::Error as ToBytesError, URefFromStrError};

/// Crate-wide Result type wrapper.
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// Error that can be returned by `casper-client`.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to parse a `Key` from a formatted string.
    #[error("failed to parse as a key")]
    FailedToParseKey,

    /// Failed to parse a `URef` from a formatted string.
    #[error("failed to parse as a uref: {0:?}")]
    FailedToParseURef(URefFromStrError),

    /// Failed to parse an integer from a string.
    #[error("failed to parse as an integer: {0:?}")]
    FailedToParseInt(#[from] ParseIntError),

    /// Failed to get a response from the node.
    #[error("failed to get rpc response: {0}")]
    FailedToGetResponse(reqwest::Error),

    /// Failed to parse the response from the node.
    #[error("failed to parse as json-rpc response: {0}")]
    FailedToParseResponse(reqwest::Error),

    /// Failed to create new key file because it already exists.
    #[error("file already exists: {0:?}")]
    FileAlreadyExists(PathBuf),

    /// Unsupported keygen algorithm.
    #[error("unsupported keygen algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// JSON-RPC error returned from the node.
    #[error("rpc response is error: {0}")]
    ResponseIsError(#[from] jsonrpc_lite::Error),

    /// Invalid JSON returned from the node.
    #[error("invalid json: {0}")]
    InvalidJson(#[from] serde_json::Error),

    /// Invalid response returned from the node.
    #[error("invalid response: {0:?}")]
    InvalidResponse(JsonRpc),

    /// Failed to send the request to the node.
    #[error("Failed sending {0:?}")]
    FailedSending(JsonRpc),

    /// Wrapper for `std::io::Error`.
    #[error("io::Error {0}")]
    IoError(#[from] std::io::Error),

    /// Failed to serialize to bytes.
    #[error("error in to_bytes {0}")]
    ToBytesError(ToBytesError),

    /// Cryptographic error.
    #[error("crypto error {0}")]
    CryptoError(#[from] CryptoError),

    /// InvalidCLValue error.
    #[error("Invalid CLValue error {0}")]
    InvalidCLValue(String),
}

impl From<URefFromStrError> for Error {
    fn from(error: URefFromStrError) -> Self {
        Error::FailedToParseURef(error)
    }
}

impl From<ToBytesError> for Error {
    fn from(error: ToBytesError) -> Self {
        Error::ToBytesError(error)
    }
}
