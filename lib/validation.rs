use std::convert::TryFrom;

use jsonrpc_lite::JsonRpc;
use thiserror::Error;

use casper_execution_engine::{
    core, core::ValidationError, storage::trie::merkle_proof::TrieMerkleProof,
};
use casper_hashing::Digest;
use casper_node::{
    rpcs::{
        chain::{BlockIdentifier, EraSummary, GetEraInfoResult},
        state::GlobalStateIdentifier,
    },
    types::json_compatibility,
};
use casper_types::{bytesrepr, Key, StoredValue, U512};

use crate::types::{self, Block, BlockHash, BlockHeader};

const GET_ITEM_RESULT_BALANCE_VALUE: &str = "balance_value";
const GET_ITEM_RESULT_STORED_VALUE: &str = "stored_value";
const GET_ITEM_RESULT_MERKLE_PROOF: &str = "merkle_proof";
const QUERY_GLOBAL_STATE_BLOCK_HEADER: &str = "block_header";

/// Error that can be returned when validating data returned from a JSON-RPC method.
#[derive(Error, Debug)]
pub enum ValidateResponseError {
    /// Failed to marshall value.
    #[error("Failed to marshall value {0}")]
    BytesRepr(bytesrepr::Error),

    /// Error from serde.
    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    /// Failed to parse JSON.
    #[error("Validate response failed to parse")]
    ValidateResponseFailedToParse,

    /// Failed to validate Merkle proofs.
    #[error(transparent)]
    ValidationError(#[from] ValidationError),

    /// The body hash in the header is not the same as the hash of the body of the block.
    #[error(
        "Block header has incorrect body hash. \
         Actual block body hash: {actual_block_body_hash:?}, \
         Block: {block:?}"
    )]
    BodyHashMismatch {
        /// The `Block` with the `BlockHeader` with the incorrect block body hash.
        block: Box<Block>,
        /// The actual hash of the block's `BlockBody`.
        actual_block_body_hash: Digest,
    },

    /// The block's hash is not the same as the header's hash.
    #[error(
        "Block has incorrect block hash. \
         Actual block body hash: {actual_block_header_hash:?}, \
         Block: {block:?}"
    )]
    BlockHashMismatch {
        /// The `Block` with the incorrect `BlockHeaderHash`
        block: Box<Block>,
        /// The actual hash of the block's `BlockHeader`
        actual_block_header_hash: BlockHash,
    },

    /// Serialized value not contained in proof.
    #[error("Serialized value not contained in proof")]
    SerializedValueNotContainedInProof,

    /// No block in response.
    #[error("No block in response")]
    NoBlockInResponse,

    /// Block hash requested does not correspond to response.
    #[error("Block hash requested does not correspond to response")]
    UnexpectedBlockHash,

    /// Block height was not as requested.
    #[error("Block height was not as requested")]
    UnexpectedBlockHeight,

    /// An invalid combination of state identifier and block header response
    #[error("Invalid combination of state identifier and block header in response")]
    InvalidGlobalStateResponse,
}

impl From<bytesrepr::Error> for ValidateResponseError {
    fn from(e: bytesrepr::Error) -> Self {
        ValidateResponseError::BytesRepr(e)
    }
}

pub(crate) fn validate_get_era_info_response(
    response: &JsonRpc,
) -> Result<(), ValidateResponseError> {
    let value = response
        .get_result()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let result: GetEraInfoResult = serde_json::from_value(value.to_owned())?;

    match result.era_summary {
        Some(EraSummary {
            state_root_hash,
            era_id,
            merkle_proof,
            stored_value,
            ..
        }) => {
            let proof_bytes = base16::decode(&merkle_proof)
                .map_err(|_| ValidateResponseError::ValidateResponseFailedToParse)?;
            let proofs: Vec<TrieMerkleProof<Key, StoredValue>> =
                bytesrepr::deserialize(proof_bytes)?;
            let key = Key::EraInfo(era_id);
            let path = &[];

            let proof_value = match stored_value {
                json_compatibility::StoredValue::EraInfo(era_info) => {
                    StoredValue::EraInfo(era_info)
                }
                _ => return Err(ValidateResponseError::ValidateResponseFailedToParse),
            };

            core::validate_query_proof(&state_root_hash, &proofs, &key, path, &proof_value)
                .map_err(Into::into)
        }
        None => Ok(()),
    }
}

pub(crate) fn validate_query_response(
    response: &JsonRpc,
    state_root_hash: &Digest,
    key: &Key,
    path: &[String],
) -> Result<(), ValidateResponseError> {
    let value = response
        .get_result()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let object = value
        .as_object()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let proofs: Vec<TrieMerkleProof<Key, StoredValue>> = {
        let proof = object
            .get(GET_ITEM_RESULT_MERKLE_PROOF)
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_str = proof
            .as_str()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_bytes = base16::decode(proof_str)
            .map_err(|_| ValidateResponseError::ValidateResponseFailedToParse)?;
        bytesrepr::deserialize(proof_bytes)?
    };

    let proof_value: &StoredValue = {
        let last_proof = proofs
            .last()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        last_proof.value()
    };

    // Here we need to validate that JSON `stored_value` is contained in the proof.
    //
    // Possible to deserialize that field into a `StoredValue` and pass below to
    // `validate_query_proof` instead of using this approach?
    {
        let value: json_compatibility::StoredValue = {
            let value = object
                .get(GET_ITEM_RESULT_STORED_VALUE)
                .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
            serde_json::from_value(value.to_owned())?
        };
        match json_compatibility::StoredValue::try_from(proof_value.clone()) {
            Ok(json_proof_value) if json_proof_value == value => (),
            _ => return Err(ValidateResponseError::SerializedValueNotContainedInProof),
        }
    }

    core::validate_query_proof(state_root_hash, &proofs, key, path, proof_value).map_err(Into::into)
}

pub(crate) fn validate_query_global_state(
    response: &JsonRpc,
    state_identifier: GlobalStateIdentifier,
    key: &Key,
    path: &[String],
) -> Result<(), ValidateResponseError> {
    let value = response
        .get_result()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let object = value
        .as_object()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let proofs: Vec<TrieMerkleProof<Key, StoredValue>> = {
        let proof = object
            .get(GET_ITEM_RESULT_MERKLE_PROOF)
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_str = proof
            .as_str()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_bytes = base16::decode(proof_str)
            .map_err(|_| ValidateResponseError::ValidateResponseFailedToParse)?;
        bytesrepr::deserialize(proof_bytes)?
    };

    let proof_value: &StoredValue = {
        let last_proof = proofs
            .last()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        last_proof.value()
    };

    let block_header_value = object
        .get(QUERY_GLOBAL_STATE_BLOCK_HEADER)
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
    let maybe_block_header: Option<BlockHeader> =
        serde_json::from_value(block_header_value.to_owned())?;

    let state_root_hash = match (state_identifier, maybe_block_header) {
        (GlobalStateIdentifier::BlockHash(_), None)
        | (GlobalStateIdentifier::StateRootHash(_), Some(_)) => {
            return Err(ValidateResponseError::InvalidGlobalStateResponse);
        }
        (GlobalStateIdentifier::BlockHash(_), Some(block_header)) => block_header.state_root_hash(),
        (GlobalStateIdentifier::StateRootHash(hash), None) => hash,
    };

    core::validate_query_proof(&state_root_hash, &proofs, key, path, proof_value)
        .map_err(Into::into)
}

pub(crate) fn validate_get_balance_response(
    response: &JsonRpc,
    state_root_hash: &Digest,
    key: &Key,
) -> Result<(), ValidateResponseError> {
    let value = response
        .get_result()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let object = value
        .as_object()
        .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;

    let balance_proof: TrieMerkleProof<Key, StoredValue> = {
        let proof = object
            .get(GET_ITEM_RESULT_MERKLE_PROOF)
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_str = proof
            .as_str()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let proof_bytes = base16::decode(proof_str)
            .map_err(|_| ValidateResponseError::ValidateResponseFailedToParse)?;
        bytesrepr::deserialize(proof_bytes)?
    };

    let balance: U512 = {
        let value = object
            .get(GET_ITEM_RESULT_BALANCE_VALUE)
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        let value_str = value
            .as_str()
            .ok_or(ValidateResponseError::ValidateResponseFailedToParse)?;
        U512::from_dec_str(value_str)
            .map_err(|_| ValidateResponseError::ValidateResponseFailedToParse)?
    };

    core::validate_balance_proof(state_root_hash, &balance_proof, *key, &balance)
        .map_err(Into::into)
}

pub(crate) fn validate_get_block_response(
    response: &JsonRpc,
    maybe_block_identifier: &Option<BlockIdentifier>,
) -> Result<(), ValidateResponseError> {
    let maybe_result = response.get_result();
    let block_value = maybe_result
        .and_then(|value| value.get("block"))
        .ok_or(ValidateResponseError::NoBlockInResponse)?;
    let maybe_block: Option<Block> = serde_json::from_value(block_value.to_owned())?;
    let block = if let Some(block) = maybe_block {
        block
    } else {
        return Ok(());
    };

    match types::validate_block_hashes_v1(&block) {
        Ok(()) => {}
        Err(v1_error) => match types::validate_block_hashes_v2(&block) {
            Ok(()) => {}
            Err(_v2_error) => return Err(v1_error),
        },
    }

    match maybe_block_identifier {
        Some(BlockIdentifier::Hash(block_hash)) => {
            if *block_hash.inner() != block.hash().inner() {
                return Err(ValidateResponseError::UnexpectedBlockHash);
            }
        }
        Some(BlockIdentifier::Height(height)) => {
            // More is necessary here to mitigate a MITM attack
            if height != &block.header().height() {
                return Err(ValidateResponseError::UnexpectedBlockHeight);
            }
        }
        // More is necessary here to mitigate a MITM attack. In this case we would want to validate
        // `block.proofs()` to make sure that 1/3 of the validator weight signed the block, and we
        // would have to know the latest validators through some trustworthy means
        None => (),
    }
    Ok(())
}
