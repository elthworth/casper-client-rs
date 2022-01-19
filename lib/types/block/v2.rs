use std::{cmp::Reverse, collections::BTreeMap};

use itertools::Itertools;

use casper_hashing::Digest;
use casper_types::{bytesrepr::ToBytes, crypto::PublicKey};

use crate::{
    types::{Block, BlockBody, BlockHash, BlockHeader, EraEnd, EraReport},
    validation::ValidateResponseError,
};

/// Cryptographically validates the hashes of the given block, using the second version of hashing.
///
/// The validation involves hashing the header to ensure it matches the claimed [`Block::hash`]
/// value, and then hashing the body to ensure it matches the claimed [`BlockHeader::body_hash`]
/// value.
///
/// For this version of hashing, the block's header and body are notionally split into chunks
/// (for the purpose of accommodating Merkle Proofs of the chunks) and the resulting hash of the
/// header and body are thus Merkle root hashes, with Blake2b hashing used throughout.
///
/// # Note
///
/// No validation of the proofs is performed in this function.
pub fn validate_hashes(block: &Block) -> Result<(), ValidateResponseError> {
    let actual_block_header_hash = hash_header(&block.header)?;
    if block.hash != actual_block_header_hash {
        return Err(ValidateResponseError::BlockHashMismatch {
            block: Box::new(block.clone()),
            actual_block_header_hash,
        });
    }

    let actual_block_body_hash = hash_body(&block.body)?;
    if block.header.body_hash != actual_block_body_hash {
        return Err(ValidateResponseError::BodyHashMismatch {
            block: Box::new(block.clone()),
            actual_block_body_hash,
        });
    }

    Ok(())
}

fn hash_header(
    BlockHeader {
        parent_hash,
        era_id,
        body_hash,
        state_root_hash,
        era_end,
        height,
        timestamp,
        protocol_version,
        random_bit,
        accumulated_seed,
    }: &BlockHeader,
) -> Result<BlockHash, ValidateResponseError> {
    let hashed_era_end = match era_end {
        None => Digest::SENTINEL_NONE,
        Some(era_end) => hash_era_end(era_end)?,
    };

    let hashed_era_id = Digest::hash(era_id.to_bytes()?);
    let hashed_height = Digest::hash(height.to_bytes()?);
    let hashed_timestamp = Digest::hash(timestamp.to_bytes()?);
    let hashed_protocol_version = Digest::hash(protocol_version.to_bytes()?);
    let hashed_random_bit = Digest::hash(random_bit.to_bytes()?);

    let digest = Digest::hash_slice_rfold(&[
        hashed_protocol_version,
        parent_hash.0,
        hashed_era_end,
        *body_hash,
        hashed_era_id,
        *state_root_hash,
        hashed_height,
        hashed_timestamp,
        hashed_random_bit,
        *accumulated_seed,
    ]);
    Ok(BlockHash::new(digest))
}

fn hash_era_end(
    EraEnd {
        next_era_validator_weights,
        era_report,
    }: &EraEnd,
) -> Result<Digest, ValidateResponseError> {
    let mut descending_validator_weight_hashed_pairs = vec![];
    for validator_weight in next_era_validator_weights
        .iter()
        .sorted_by_key(|validator_weight| Reverse(validator_weight.weight()))
    {
        let validator_hash = Digest::hash(validator_weight.validator().to_bytes()?);
        let weight_hash = Digest::hash(validator_weight.weight().to_bytes()?);
        descending_validator_weight_hashed_pairs
            .push(Digest::hash_pair(&validator_hash, &weight_hash));
    }
    let hashed_next_era_validator_weights =
        Digest::hash_merkle_tree(descending_validator_weight_hashed_pairs);
    let hashed_era_report = hash_era_report(era_report)?;
    Ok(Digest::hash_slice_rfold(&[
        hashed_next_era_validator_weights,
        hashed_era_report,
    ]))
}

fn hash_era_report(
    EraReport {
        equivocators,
        inactive_validators,
        rewards,
    }: &EraReport,
) -> Result<Digest, ValidateResponseError> {
    let hashed_equivocators = hash_slice_of_public_keys(equivocators)?;
    let hashed_inactive_validators = hash_slice_of_public_keys(inactive_validators)?;
    let rewards: BTreeMap<PublicKey, u64> = rewards
        .iter()
        .map(|reward| (reward.validator().clone(), reward.amount()))
        .collect();

    let hashed_rewards = Digest::hash_btree_map(&rewards)?;

    Ok(Digest::hash_slice_rfold(&[
        hashed_equivocators,
        hashed_rewards,
        hashed_inactive_validators,
    ]))
}

fn hash_slice_of_public_keys(public_keys: &[PublicKey]) -> Result<Digest, ValidateResponseError> {
    let serialized_public_keys = public_keys
        .iter()
        .map(|validator| validator.to_bytes())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Digest::hash_merkle_tree(
        serialized_public_keys.iter().map(Digest::hash),
    ))
}

fn hash_body(
    BlockBody {
        deploy_hashes,
        transfer_hashes,
        proposer,
    }: &BlockBody,
) -> Result<Digest, ValidateResponseError> {
    let proposer_digest =
        Digest::hash_pair(Digest::hash(&proposer.to_bytes()?), Digest::SENTINEL_RFOLD);

    let transfer_hashes_digest = Digest::hash_pair(
        Digest::hash_merkle_tree(transfer_hashes.iter().cloned().map(Digest::from)),
        proposer_digest,
    );

    let deploy_hashes_digest = Digest::hash_pair(
        Digest::hash_merkle_tree(deploy_hashes.iter().cloned().map(Digest::from)),
        transfer_hashes_digest,
    );

    Ok(deploy_hashes_digest)
}
