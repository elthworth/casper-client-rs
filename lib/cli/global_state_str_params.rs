use casper_hashing::Digest;

use crate::{cli::CliError, rpcs::common::GlobalStateIdentifier, types::BlockHash};

#[derive(Debug)]
/// The ways to identify the string value.
pub enum GlobalStateStrIdentifier {
    /// The global state identifier is either a state root hash or a block hash.
    Hash {
        /// Whether the hash is a block hash or state root hash.
        is_block_hash: bool,
    },
    /// The global state identifier is the block height.
    Height,
}

/// The ways to construct a query to global state.
#[derive(Debug)]
pub struct GlobalStateStrParams<'a> {
    /// Whether the identifier is the block height, block hash or the state root hash.
    pub str_identifier: GlobalStateStrIdentifier,
    /// The identifier value.
    pub identifier_value: &'a str,
}

impl<'a> TryFrom<GlobalStateStrParams<'a>> for GlobalStateIdentifier {
    type Error = CliError;

    fn try_from(params: GlobalStateStrParams<'a>) -> Result<GlobalStateIdentifier, Self::Error> {
        match params.str_identifier {
            GlobalStateStrIdentifier::Hash { is_block_hash } => {
                let hash = Digest::from_hex(params.identifier_value).map_err(|error| {
                    CliError::FailedToParseDigest {
                        context: "global_state_identifier",
                        error,
                    }
                })?;

                if is_block_hash {
                    Ok(GlobalStateIdentifier::BlockHash(BlockHash::new(hash)))
                } else {
                    Ok(GlobalStateIdentifier::StateRootHash(hash))
                }
            }
            GlobalStateStrIdentifier::Height => {
                let height = params.identifier_value.parse().map_err(|error| {
                    CliError::FailedToParseInt {
                        context: "global_state_identifier",
                        error,
                    }
                })?;
                Ok(GlobalStateIdentifier::BlockHeight(height))
            }
        }
    }
}
