#[cfg(doc)]
use casper_types::{account::AccountHash, HashAddr};
use casper_types::{Key, URef};

#[cfg(doc)]
use crate::types::{Account, Contract};
use crate::{cli::CliError, rpcs::DictionaryItemIdentifier};

/// Various ways of uniquely identifying a dictionary item.
pub enum DictionaryItemStrParams<'a> {
    /// A dictionary item identified via an [`Account`]'s named keys.
    AccountNamedKey {
        /// The [`AccountHash`] as a formatted string, identifying the account whose named keys
        /// contains `dictionary_name`.
        account_hash: &'a str,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: &'a str,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: &'a str,
    },
    /// A dictionary item identified via a [`Contract`]'s named keys.
    ContractNamedKey {
        /// The [`HashAddr`] as a formatted string, identifying the contract whose named keys
        /// contains `dictionary_name`.
        hash_addr: &'a str,
        /// The named key under which the dictionary seed `URef` is stored.
        dictionary_name: &'a str,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: &'a str,
    },
    /// A dictionary item identified via its seed [`URef`].
    URef {
        /// The dictionary's seed `URef` as a formatted string.
        seed_uref: &'a str,
        /// The key within the dictionary under which the item is held.
        dictionary_item_key: &'a str,
    },
    /// A dictionary item identified via its unique address derived from the dictionary's seed
    /// `URef` and the item's key within the dictionary.  The key must be a `Key::Dictionary`
    /// variant as a formatted string.
    Dictionary(&'a str),
}

impl<'a> TryFrom<DictionaryItemStrParams<'a>> for DictionaryItemIdentifier {
    type Error = CliError;

    fn try_from(
        params: DictionaryItemStrParams<'a>,
    ) -> Result<DictionaryItemIdentifier, Self::Error> {
        match params {
            DictionaryItemStrParams::AccountNamedKey {
                account_hash,
                dictionary_name,
                dictionary_item_key,
            } => {
                let key = Key::from_formatted_str(account_hash).map_err(|error| {
                    CliError::FailedToParseKey {
                        context: "dictionary item account named key",
                        error,
                    }
                })?;
                let account_hash = key.into_account().ok_or(CliError::InvalidArgument {
                    context: "dictionary item account named key",
                    error: "not an account hash".to_string(),
                })?;
                Ok(DictionaryItemIdentifier::new_from_account_info(
                    account_hash,
                    dictionary_name.to_string(),
                    dictionary_item_key.to_string(),
                ))
            }
            DictionaryItemStrParams::ContractNamedKey {
                hash_addr,
                dictionary_name,
                dictionary_item_key,
            } => {
                let key = Key::from_formatted_str(hash_addr).map_err(|error| {
                    CliError::FailedToParseKey {
                        context: "dictionary item contract named key",
                        error,
                    }
                })?;
                let hash_addr = key.into_hash().ok_or(CliError::InvalidArgument {
                    context: "dictionary item contract named key",
                    error: "not a hash-addr".to_string(),
                })?;
                Ok(DictionaryItemIdentifier::new_from_contract_info(
                    hash_addr,
                    dictionary_name.to_string(),
                    dictionary_item_key.to_string(),
                ))
            }
            DictionaryItemStrParams::URef {
                seed_uref,
                dictionary_item_key,
            } => {
                let seed_uref = URef::from_formatted_str(seed_uref).map_err(|error| {
                    CliError::FailedToParseURef {
                        context: "dictionary item uref",
                        error,
                    }
                })?;
                Ok(DictionaryItemIdentifier::new_from_seed_uref(
                    seed_uref,
                    dictionary_item_key.to_string(),
                ))
            }
            DictionaryItemStrParams::Dictionary(address) => {
                let key = Key::from_formatted_str(address).map_err(|error| {
                    CliError::FailedToParseKey {
                        context: "dictionary item address",
                        error,
                    }
                })?;
                Ok(DictionaryItemIdentifier::new_from_item_key(key)?)
            }
        }
    }
}
