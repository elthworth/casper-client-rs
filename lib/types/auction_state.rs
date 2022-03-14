//! Types associated with reporting auction state.

mod bid;
mod delegator;
mod era_validators;
mod validator_weight;

use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_hashing::Digest;

pub use bid::{Bid, BidderAndBid};
pub use delegator::Delegator;
pub use era_validators::EraValidators;
pub use validator_weight::ValidatorWeight;

/// The state associated with the auction system contract as at the given block height and
/// corresponding state root hash.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuctionState {
    state_root_hash: Digest,
    block_height: u64,
    era_validators: Vec<EraValidators>,
    bids: Vec<BidderAndBid>,
}

impl AuctionState {
    /// Returns the state root hash applicable to this auction state.
    pub fn state_root_hash(&self) -> &Digest {
        &self.state_root_hash
    }

    /// Returns the block height applicable to this auction state.
    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    /// Returns the validators for the applicable era.
    pub fn era_validators(&self) -> impl Iterator<Item = &EraValidators> {
        self.era_validators.iter()
    }

    /// Returns the bids for the applicable era.
    pub fn bids(&self) -> impl Iterator<Item = &BidderAndBid> {
        self.bids.iter()
    }
}

impl Display for AuctionState {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "auction state {{ state root hash {}, block height {}, validators {{{}}}, bids {{{}}} \
            }}",
            self.state_root_hash,
            self.block_height,
            self.era_validators().format(", "),
            self.bids().format(", "),
        )
    }
}
