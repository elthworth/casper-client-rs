use crate::cli::CliError;
use crate::rpcs::v1_5_0::query_balance::PurseIdentifier;
use casper_types::account::AccountHash;
use casper_types::{AsymmetricType, PublicKey, URef};

#[derive(Debug)]
/// The ways to construct a PurseIdentifier from a string.
pub enum PurseStrIdentifier {
    /// The string value is to be parsed as a PublicKey.
    PublicKey,
    /// The string value is to be parsed as an AccountHash.
    AccountHash,
    /// The string value is to be parsed as an URef.
    PurseURef,
}

/// The ways to construct a balance query.
#[derive(Debug)]
pub struct PurseStrParams {
    /// The identifier for the string value.
    pub purse_str_identifier: PurseStrIdentifier,
    /// The identifier value to parsed.
    pub identifier_value: String,
}

impl TryFrom<PurseStrParams> for PurseIdentifier {
    type Error = CliError;

    fn try_from(params: PurseStrParams) -> Result<PurseIdentifier, Self::Error> {
        match params.purse_str_identifier {
            PurseStrIdentifier::PublicKey => {
                let account_public_key =
                    PublicKey::from_hex(params.identifier_value).map_err(|error| {
                        CliError::FailedToParsePublicKey {
                            context: "purse_identifier",
                            error,
                        }
                    })?;
                Ok(PurseIdentifier::MainPurseUnderPublicKey(account_public_key))
            }
            PurseStrIdentifier::AccountHash => {
                let account_hash = AccountHash::from_formatted_str(&*params.identifier_value)
                    .map_err(|error| CliError::FailedToParseKey {
                        context: "purse_identifier",
                        error: error.into(),
                    })?;
                Ok(PurseIdentifier::MainPurseUnderAccountHash(account_hash))
            }
            PurseStrIdentifier::PurseURef => {
                let uref =
                    URef::from_formatted_str(&*params.identifier_value).map_err(|error| {
                        CliError::FailedToParseURef {
                            context: "purse_identifier",
                            error,
                        }
                    })?;
                Ok(PurseIdentifier::PurseUref(uref))
            }
        }
    }
}
