use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_types::{account::AccountHash, URef};

#[cfg(doc)]
use crate::types::Contract;
use crate::types::NamedKey;

/// A representation of a public key and weight which can be associated with a given [`Account`] or
/// [`Contract`].
///
/// Note, the "key" in this case is represented by an [`AccountHash`].
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AssociatedKey {
    account_hash: AccountHash,
    weight: u8,
}

impl AssociatedKey {
    /// Returns the account hash of this associated key.
    pub fn account_hash(&self) -> &AccountHash {
        &self.account_hash
    }

    /// Returns the weight attributed to this associated key.
    pub fn weight(&self) -> u8 {
        self.weight
    }
}

impl Display for AssociatedKey {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "associated-key {{ {}, weight: {} }}",
            self.account_hash, self.weight
        )
    }
}

/// Thresholds that have to be met when executing an action of a certain type.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionThresholds {
    deployment: u8,
    key_management: u8,
}

impl ActionThresholds {
    /// Returns the deployment threshold.
    pub fn deployment(&self) -> u8 {
        self.deployment
    }

    /// Returns the key-management threshold.
    pub fn key_management(&self) -> u8 {
        self.key_management
    }
}

impl Display for ActionThresholds {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "action-thresholds {{ deployment: {}, key-management: {} }}",
            self.deployment, self.key_management
        )
    }
}

/// Structure representing a user's account, stored in global state.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Account {
    account_hash: AccountHash,
    named_keys: Vec<NamedKey>,
    main_purse: URef,
    associated_keys: Vec<AssociatedKey>,
    action_thresholds: ActionThresholds,
}

impl Account {
    /// Returns the account hash (the hash of the public key) of the owner of the account.
    pub fn account_hash(&self) -> &AccountHash {
        &self.account_hash
    }

    /// Returns the named keys of the account.
    pub fn named_keys(&self) -> impl Iterator<Item = &NamedKey> {
        self.named_keys.iter()
    }

    /// Returns the main purse of the account.
    pub fn main_purse(&self) -> &URef {
        &self.main_purse
    }

    /// Returns the associated-keys of the account.
    pub fn associated_keys(&self) -> impl Iterator<Item = &AssociatedKey> {
        self.associated_keys.iter()
    }

    /// Returns the action-thresholds of the account.
    pub fn action_thresholds(&self) -> ActionThresholds {
        self.action_thresholds
    }
}

impl Display for Account {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "account {{ {}, main-purse {}, {}, named keys {{{}}}, associated-keys {{{}}} }}",
            self.account_hash,
            self.main_purse,
            self.action_thresholds,
            self.named_keys.iter().format(", "),
            self.associated_keys.iter().format(", ")
        )
    }
}
