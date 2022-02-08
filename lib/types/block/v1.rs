use casper_hashing::Digest;
use casper_types::bytesrepr::ToBytes;

#[cfg(doc)]
use crate::types::BlockHeader;
use crate::{
    types::{Block, BlockHash},
    validation::ValidateResponseError,
};

/// Cryptographically validates the hashes of the given block, using the initial version of hashing.
///
/// The validation involves hashing the serialized header to ensure it matches the claimed
/// [`Block::hash`] value, and then hashing the serialized body to ensure it matches the claimed
/// [`BlockHeader::body_hash`] value.
///
/// For this version of hashing, a simple Blake2b hash of the serialized data is performed.
///
/// # Note
///
/// No validation of the proofs is performed in this function.
pub fn validate_hashes(block: &Block) -> Result<(), ValidateResponseError> {
    let serialized_header = block.header.to_bytes()?;
    let actual_block_header_hash = BlockHash::new(Digest::hash(&serialized_header));
    if block.hash != actual_block_header_hash {
        return Err(ValidateResponseError::BlockHashMismatch {
            block: Box::new(block.clone()),
            actual_block_header_hash,
        });
    }

    let serialized_body = block.body.to_bytes()?;
    let actual_block_body_hash = Digest::hash(&serialized_body);
    if block.header.body_hash != actual_block_body_hash {
        return Err(ValidateResponseError::BodyHashMismatch {
            block: Box::new(block.clone()),
            actual_block_body_hash,
        });
    }

    Ok(())
}
