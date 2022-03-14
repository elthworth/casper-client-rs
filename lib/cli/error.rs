use std::num::ParseIntError;

use humantime::{DurationError, TimestampError};
use thiserror::Error;

use casper_types::{CLValueError, KeyFromStrError, UIntParseError, URefFromStrError};
#[cfg(doc)]
use casper_types::{Key, PublicKey, URef};

#[cfg(doc)]
use crate::{
    rpcs::{DictionaryItemIdentifier, GlobalStateIdentifier},
    types::{TimeDiff, Timestamp},
};

/// Error that can be returned by the `cli` API.
#[derive(Error, Debug)]
pub enum CliError {
    /// Failed to parse a [`Key`] from a formatted string.
    #[error("failed to parse {context} as a key: {error}")]
    FailedToParseKey {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// The actual error raised.
        error: KeyFromStrError,
    },

    /// Failed to parse a [`PublicKey`] from a formatted string.
    #[error("failed to parse {context} as a key: {error}")]
    FailedToParsePublicKey {
        /// Contextual description of where this error occurred.
        context: &'static str,
        /// The actual error raised.
        error: casper_types::crypto::Error,
    },

    /// Failed to parse a [`URef`] from a formatted string.
    #[error("failed to parse '{context}' as a uref: {error}")]
    FailedToParseURef {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: URefFromStrError,
    },

    /// Failed to parse an integer from a string.
    #[error("failed to parse '{context}' as an integer: {error}")]
    FailedToParseInt {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: ParseIntError,
    },

    /// Failed to parse a [`TimeDiff`] from a formatted string.
    #[error("failed to parse '{context}' as a time diff: {error}")]
    FailedToParseTimeDiff {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: DurationError,
    },

    /// Failed to parse a [`Timestamp`] from a formatted string.
    #[error("failed to parse '{context}' as a timestamp: {error}")]
    FailedToParseTimestamp {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: TimestampError,
    },

    /// Failed to parse a `U128`, `U256` or `U512` from a string.
    #[error("failed to parse '{context}' as u128, u256, or u512: {error:?}")]
    FailedToParseUint {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: UIntParseError,
    },

    /// Failed to parse a `Digest` from a string.
    #[error("failed to parse '{context}' as a hash digest: {error:?}")]
    FailedToParseDigest {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// The actual error raised.
        error: casper_hashing::Error,
    },

    /// Failed to create a [`GlobalStateIdentifier`].
    #[error("failed to parse state identifier")]
    FailedToParseStateIdentifier,

    /// Conflicting arguments.
    #[error("conflicting arguments passed '{context}' {args:?}")]
    ConflictingArguments {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// Arguments passed, with their values.
        args: Vec<String>,
    },

    /// Invalid `CLValue`.
    #[error("invalid CLValue error: {0}")]
    InvalidCLValue(String),

    /// Invalid argument.
    #[error("invalid argument '{context}': {error}")]
    InvalidArgument {
        /// Contextual description of where this error occurred including relevant paths,
        /// filenames, etc.
        context: &'static str,
        /// An error message.
        error: String,
    },

    /// Core error.
    #[error(transparent)]
    Core(#[from] crate::Error),
}

impl From<CLValueError> for CliError {
    fn from(error: CLValueError) -> Self {
        match error {
            CLValueError::Serialization(bytesrepr_error) => CliError::Core(bytesrepr_error.into()),
            CLValueError::Type(type_mismatch) => {
                CliError::InvalidCLValue(type_mismatch.to_string())
            }
        }
    }
}
