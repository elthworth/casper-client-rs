use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_types::{PublicKey, URef, U512};

use super::Delegator;

/// A representation of a single bid, containing most of the info in
/// [`casper_types::system::auction::Bid`].
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Bid {
    bonding_purse: URef,
    staked_amount: U512,
    delegation_rate: u8,
    delegators: Vec<Delegator>,
    inactive: bool,
}

impl Bid {
    /// Returns the purse that was used for bonding.
    pub fn bonding_purse(&self) -> &URef {
        &self.bonding_purse
    }

    /// Returns the amount of tokens staked by a validator (not including delegators).
    pub fn staked_amount(&self) -> U512 {
        self.staked_amount
    }

    /// Returns the validator's specified delegation rate.
    pub fn delegation_rate(&self) -> u8 {
        self.delegation_rate
    }

    /// Returns the delegators associated with this validator's bid.
    pub fn delegators(&self) -> impl Iterator<Item = &Delegator> {
        self.delegators.iter()
    }

    /// Returns `true` if this validator has been evicted and is hence considered inactive.
    pub fn inactive(&self) -> &bool {
        &self.inactive
    }
}

impl Display for Bid {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "bid {{ bonding purse {}, staked {}, delegation rate {}, delegators {{{}}}, is \
            {}inactive }}",
            self.bonding_purse,
            self.staked_amount,
            self.delegation_rate,
            self.delegators().format(", "),
            if self.inactive { "" } else { "not " }
        )
    }
}

/// A pair of bidder's public key and the associated bid.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BidderAndBid {
    public_key: PublicKey,
    bid: Bid,
}

impl BidderAndBid {
    /// Returns the bidder's public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Returns the bid.
    pub fn bid(&self) -> &Bid {
        &self.bid
    }
}

impl Display for BidderAndBid {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{{ bidder {}, {} }}", self.public_key, self.bid,)
    }
}
