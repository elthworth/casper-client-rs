use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use casper_types::{PublicKey, URef, U512};

/// A delegator associated with the given validator.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Delegator {
    public_key: PublicKey,
    staked_amount: U512,
    bonding_purse: URef,
    delegatee: PublicKey,
}

impl Delegator {
    /// Returns the public key of the delegator.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Returns the staked amount of the delegator.
    pub fn staked_amount(&self) -> U512 {
        self.staked_amount
    }

    /// Returns the bonding purse of the delegator.
    pub fn bonding_purse(&self) -> &URef {
        &self.bonding_purse
    }

    /// Returns the delegatee of the delegator, i.e. the validator to which this delegation applies.
    pub fn delegatee(&self) -> &PublicKey {
        &self.delegatee
    }
}

impl Display for Delegator {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "delegator {{ {} {} motes, bonding purse {}, delegatee {} }}",
            self.public_key, self.staked_amount, self.bonding_purse, self.delegatee
        )
    }
}
