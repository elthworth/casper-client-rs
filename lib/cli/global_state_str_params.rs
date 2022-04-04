use casper_hashing::Digest;

use crate::{cli::CliError, rpcs::GlobalStateIdentifier, types::BlockHash};

/// The two ways to construct a query to global state.
#[derive(Default, Debug)]
pub struct GlobalStateStrParams<'a> {
    /// Whether the hash is a block hash or state root hash.
    pub is_block_hash: bool,
    /// The hex-encoded hash value.
    pub hash_value: &'a str,
}

impl<'a> TryFrom<GlobalStateStrParams<'a>> for GlobalStateIdentifier {
    type Error = CliError;

    fn try_from(params: GlobalStateStrParams<'a>) -> Result<GlobalStateIdentifier, Self::Error> {
        let hash =
            Digest::from_hex(params.hash_value).map_err(|error| CliError::FailedToParseDigest {
                context: "global_state_identifier",
                error,
            })?;

        if params.is_block_hash {
            Ok(GlobalStateIdentifier::BlockHash(BlockHash::new(hash)))
        } else {
            Ok(GlobalStateIdentifier::StateRootHash(hash))
        }
    }
}
