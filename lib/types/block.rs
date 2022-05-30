pub mod v1;
pub mod v2;

use std::fmt::{self, Display, Formatter};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{
    bytesrepr::{self, ToBytes},
    crypto::PublicKey,
    EraId, ProtocolVersion,
};

#[cfg(doc)]
use crate::types::{validate_block_hashes_v1, validate_block_hashes_v2};
use crate::types::{DeployHash, EraEnd, Proof, Timestamp};

/// A cryptographic hash uniquely identifying a [`Block`].
///
/// # Note
///
/// The type of this field is currently the same for all versions of blocks, and furthermore it is
/// always a hash over the block's header.  However, *how* the hash is calculated can be different.
///
/// There are two separate functions to allow for validation of the block given the different
/// hash mechanisms: [`validate_block_hashes_v1`] and [`validate_block_hashes_v2`].
#[derive(
    Copy,
    Clone,
    Default,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Debug,
    JsonSchema,
)]
#[serde(deny_unknown_fields)]
pub struct BlockHash(Digest);

impl BlockHash {
    /// Returns a new `BlockHash`.
    pub fn new(digest: Digest) -> Self {
        BlockHash(digest)
    }

    /// Returns a copy of the wrapped `Digest`.
    pub fn inner(&self) -> Digest {
        self.0
    }
}

impl Display for BlockHash {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl ToBytes for BlockHash {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        self.0.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        self.0.to_bytes()
    }

    fn serialized_length(&self) -> usize {
        self.0.serialized_length()
    }
}

/// The header portion of a [`Block`].
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockHeader {
    parent_hash: BlockHash,
    state_root_hash: Digest,
    body_hash: Digest,
    random_bit: bool,
    accumulated_seed: Digest,
    era_end: Option<EraEnd>,
    timestamp: Timestamp,
    era_id: EraId,
    height: u64,
    protocol_version: ProtocolVersion,
}

impl BlockHeader {
    /// Returns the parent block's hash.
    pub fn parent_hash(&self) -> BlockHash {
        self.parent_hash
    }

    /// Returns the root hash of global state after executing the deploys in this block.
    pub fn state_root_hash(&self) -> Digest {
        self.state_root_hash
    }

    /// Returns the hash of the body of this block.
    ///
    /// # Note
    ///
    /// See [`validate_block_hashes_v1`] and [`validate_block_hashes_v2`] for further details on the
    /// different ways in which this hash can be calculated.
    pub fn body_hash(&self) -> Digest {
        self.body_hash
    }

    /// Returns the random bit held in this block.
    pub fn random_bit(&self) -> bool {
        self.random_bit
    }

    /// Returns the accumulated seed held in this block.
    pub fn accumulated_seed(&self) -> Digest {
        self.accumulated_seed
    }

    /// Returns the `EraEnd`, where `Some` means this is a "switch block", i.e. the final block of
    /// the specified era.
    pub fn era_end(&self) -> Option<&EraEnd> {
        self.era_end.as_ref()
    }

    /// Returns the creation timestamp of this block.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    /// Returns the ID of the era in which this block belongs.
    pub fn era_id(&self) -> EraId {
        self.era_id
    }

    /// Returns the height of the block in the blockchain.
    pub fn height(&self) -> u64 {
        self.height
    }

    /// Returns the protocol version of the network at the time this block was created.
    pub fn protocol_version(&self) -> ProtocolVersion {
        self.protocol_version
    }
}

impl Display for BlockHeader {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "block header {{ parent hash {}, post-state hash {}, body hash {}, \
            random bit {}, accumulated seed {}, timestamp {} }}",
            self.parent_hash.0,
            self.state_root_hash,
            self.body_hash,
            self.random_bit,
            self.accumulated_seed,
            self.timestamp,
        )?;
        if let Some(era_end) = &self.era_end {
            write!(formatter, ", era_end {}", era_end)?;
        }
        Ok(())
    }
}

impl ToBytes for BlockHeader {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        self.parent_hash.write_bytes(buffer)?;
        self.state_root_hash.write_bytes(buffer)?;
        self.body_hash.write_bytes(buffer)?;
        self.random_bit.write_bytes(buffer)?;
        self.accumulated_seed.write_bytes(buffer)?;
        self.era_end.write_bytes(buffer)?;
        self.timestamp.write_bytes(buffer)?;
        self.era_id.write_bytes(buffer)?;
        self.height.write_bytes(buffer)?;
        self.protocol_version.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.parent_hash.serialized_length()
            + self.state_root_hash.serialized_length()
            + self.body_hash.serialized_length()
            + self.random_bit.serialized_length()
            + self.accumulated_seed.serialized_length()
            + self.era_end.serialized_length()
            + self.timestamp.serialized_length()
            + self.era_id.serialized_length()
            + self.height.serialized_length()
            + self.protocol_version.serialized_length()
    }
}

/// The body portion of a block.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BlockBody {
    proposer: PublicKey,
    deploy_hashes: Vec<DeployHash>,
    transfer_hashes: Vec<DeployHash>,
}

impl BlockBody {
    /// Returns the public key of the validator which proposed this block.
    pub fn proposer(&self) -> &PublicKey {
        &self.proposer
    }

    /// Returns the hashes of all non-transfer deploys included in this block.
    pub fn deploy_hashes(&self) -> impl Iterator<Item = &DeployHash> {
        self.deploy_hashes.iter()
    }

    /// Returns the hashes of all transfers included in this block.
    pub fn transfer_hashes(&self) -> impl Iterator<Item = &DeployHash> {
        self.transfer_hashes.iter()
    }
}

impl Display for BlockBody {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self)
    }
}

impl ToBytes for BlockBody {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        self.proposer.write_bytes(buffer)?;
        self.deploy_hashes.write_bytes(buffer)?;
        self.transfer_hashes.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.proposer.serialized_length()
            + self.deploy_hashes.serialized_length()
            + self.transfer_hashes.serialized_length()
    }
}

/// A block; the core component of the Casper linear blockchain.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Block {
    hash: BlockHash,
    header: BlockHeader,
    body: BlockBody,
    proofs: Vec<Proof>,
}

impl Block {
    /// Returns the hash uniquely identifying this block.
    ///
    /// # Note
    ///
    /// See [`validate_block_hashes_v1`] and [`validate_block_hashes_v2`] for further details on the
    /// different ways in which this hash can be calculated.
    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    /// Returns the header portion of the block.
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    /// Returns the body portion of the block.
    pub fn body(&self) -> &BlockBody {
        &self.body
    }

    /// Returns the proofs; the public keys and signatures of the validators which signed the block.
    pub fn proofs(&self) -> impl Iterator<Item = &Proof> {
        self.proofs.iter()
    }
}

impl Display for Block {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "block {{ {}, parent hash {}, post-state hash {}, body hash {}, \
             random bit {}, timestamp {}, {}, height {}, protocol version: {} }}",
            self.hash,
            self.header.parent_hash,
            self.header.state_root_hash,
            self.header.body_hash,
            self.header.random_bit,
            self.header.timestamp,
            self.header.era_id,
            self.header.height,
            self.header.protocol_version
        )?;
        if let Some(era_end) = &self.header.era_end {
            write!(formatter, ", era-end: {}", era_end)?;
        }
        Ok(())
    }
}

impl ToBytes for Block {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        // Note: the proofs are deliberately not part of the bytesrepr serialized block.
        self.hash.write_bytes(buffer)?;
        self.header.write_bytes(buffer)?;
        self.body.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.hash.serialized_length()
            + self.header.serialized_length()
            + self.body.serialized_length()
    }
}

/// Describes a block's hash and height.
#[derive(Clone, Copy, Default, Eq, JsonSchema, Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHashAndHeight {
    /// The hash of the block.
    #[schemars(description = "The hash of this deploy's block.")]
    pub block_hash: BlockHash,
    /// The height of the block.
    #[schemars(description = "The height of this deploy's block.")]
    pub block_height: u64,
}
