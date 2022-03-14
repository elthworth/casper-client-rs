use serde::{Deserialize, Serialize};

use casper_hashing::Digest;
use casper_types::{account::AccountHash, HashAddr, Key, ProtocolVersion, URef};

#[cfg(doc)]
use crate::types::{Account, Contract};
use crate::{types::StoredValue, Error};

pub(crate) const GET_DICTIONARY_ITEM_METHOD: &str = "state_get_dictionary_item";

/// The identifier for a dictionary item.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum DictionaryItemIdentifier {
    /// A dictionary item identified via an [`Account`]'s named keys.
    AccountNamedKey {
        /// The [`Key::Account`] as a formatted string, identifying the account whose named keys
        /// contains `dictionary_name`.
        key: String,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: String,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via a [`Contract`]'s named keys.
    ContractNamedKey {
        /// The [`Key::Hash`] as a formatted string, identifying the contract whose named keys
        /// contains `dictionary_name`.
        key: String,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: String,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via its seed [`URef`].
    URef {
        /// The dictionary's seed `URef`.
        seed_uref: URef,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: String,
    },
    /// A dictionary item identified via its unique address derived from the dictionary's seed
    /// `URef` and the item's key within the dictionary.  The key must be a `Key::Dictionary`
    /// variant, as a formatted string.
    Dictionary(String),
}

impl DictionaryItemIdentifier {
    /// Returns a new `DictionaryItemIdentifier::AccountNamedKey` variant.
    pub fn new_from_account_info(
        account_hash: AccountHash,
        dictionary_name: String,
        dictionary_item_key: String,
    ) -> Self {
        DictionaryItemIdentifier::AccountNamedKey {
            key: Key::Account(account_hash).to_formatted_string(),
            dictionary_name,
            dictionary_item_key,
        }
    }

    /// Returns a new `DictionaryItemIdentifier::ContractNamedKey` variant.
    pub fn new_from_contract_info(
        contract_addr: HashAddr,
        dictionary_name: String,
        dictionary_item_key: String,
    ) -> Self {
        DictionaryItemIdentifier::ContractNamedKey {
            key: Key::Hash(contract_addr).to_formatted_string(),
            dictionary_name,
            dictionary_item_key,
        }
    }

    /// Returns a new `DictionaryItemIdentifier::URef` variant.
    pub fn new_from_seed_uref(seed_uref: URef, dictionary_item_key: String) -> Self {
        DictionaryItemIdentifier::URef {
            seed_uref,
            dictionary_item_key,
        }
    }

    /// Returns a new `DictionaryItemIdentifier::Dictionary` variant.
    pub fn new_from_item_key(item_key: Key) -> Result<Self, Error> {
        if item_key.as_dictionary().is_some() {
            Ok(DictionaryItemIdentifier::Dictionary(
                item_key.to_formatted_string(),
            ))
        } else {
            Err(Error::InvalidKeyVariant {
                expected_variant: "Key::Dictionary".to_string(),
                actual: item_key,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct GetDictionaryItemParams {
    state_root_hash: Digest,
    dictionary_identifier: DictionaryItemIdentifier,
}

impl GetDictionaryItemParams {
    pub(crate) fn new(
        state_root_hash: Digest,
        dictionary_identifier: DictionaryItemIdentifier,
    ) -> Self {
        GetDictionaryItemParams {
            state_root_hash,
            dictionary_identifier,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `state_get_dictionary_item` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetDictionaryItemResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The dictionary key under which the value is stored.
    pub dictionary_key: String,
    /// The stored value.
    pub stored_value: StoredValue,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
