//! Various data types of the Casper network.

mod block;
mod era_end;
mod proof;
mod timestamp;

pub use block::{
    v1::validate_hashes as validate_block_hashes_v1,
    v2::validate_hashes as validate_block_hashes_v2, Block, BlockBody, BlockHash, BlockHeader,
};
pub use era_end::{EraEnd, EraReport, Reward, ValidatorWeight};
pub use proof::Proof;
pub use timestamp::Timestamp;
