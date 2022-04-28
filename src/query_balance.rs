use std::{fs, str};

use async_trait::async_trait;
use clap::{Arg, ArgGroup, ArgMatches, Command};

use casper_client::cli::{
    CliError, GlobalStateStrIdentifier, GlobalStateStrParams, PurseStrIdentifier, PurseStrParams,
};

use crate::{command::ClientCommand, common, Success};

const ARG_HEX_STRING: &str = "HEX STRING";

pub struct QueryBalance;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockHash,
    BlockHeight,
    StateRootHash,
    PublicKey,
    AccountHash,
    PurseURef,
}

mod state_root_hash {
    use super::*;

    pub(super) const ARG_NAME: &str = "state-root-hash";
    const ARG_SHORT: char = 's';
    const ARG_VALUE_NAME: &str = ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the state root";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::StateRootHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.value_of(ARG_NAME)
    }
}

mod block_hash {
    use super::*;

    pub(super) const ARG_NAME: &str = "block-hash";
    const ARG_SHORT: char = 'b';
    const ARG_VALUE_NAME: &str = ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the block";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::BlockHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.value_of(ARG_NAME)
    }
}

mod block_height {
    use super::*;

    pub(super) const ARG_NAME: &str = "block-height";
    const ARG_VALUE_NAME: &str = "INTEGER";
    const ARG_HELP: &str = "Height of the block";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::BlockHeight as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.value_of(ARG_NAME)
    }
}

mod public_key {
    use casper_types::{AsymmetricType, PublicKey};

    use super::*;
    use casper_client::{cli::CliError, AsymmetricKeyExt};

    pub const ARG_NAME: &str = "public-key";
    const ARG_SHORT: char = 'p';
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "This must be a properly formatted public key. The public key may instead be read in from \
        a file, in which case enter the path to the file as the --public-key argument. The file \
        should be one of the two public key files generated via the `keygen` subcommand; \
        \"public_key_hex\" or \"public_key.pem\"";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PublicKey as usize)
    }

    pub(super) fn get(matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches.value_of(ARG_NAME).unwrap_or_default();

        // Try to read as a PublicKey PEM file first.
        if let Ok(public_key) = PublicKey::from_file(value) {
            return Ok(public_key.to_hex());
        }

        // Try to read as a hex-encoded PublicKey file next.
        if let Ok(hex_public_key) = fs::read_to_string(value) {
            let _ = PublicKey::from_hex(&hex_public_key).map_err(|error| {
                eprintln!(
                    "Can't parse the contents of {} as a public key: {}",
                    value, error
                );
                CliError::FailedToParsePublicKey {
                    context: "get public key",
                    error,
                }
            })?;
            return Ok(hex_public_key);
        }
        Ok(value.to_string())
    }
}

mod account_hash {
    use super::*;

    pub(super) const ARG_NAME: &str = "account-hash";
    const ARG_SHORT: char = 'a';
    const ARG_VALUE_NAME: &str = "FORMATTED STRING";
    const ARG_HELP: &str =
        "This must be a properly formatted account hash. The format for account hash is \
        \"account-hash-<HEX STRING>\".";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::AccountHash as usize)
    }

    pub fn get(matches: &ArgMatches) -> Option<&str> {
        matches.value_of(ARG_NAME)
    }
}

fn maybe_global_state_str_params(matches: &ArgMatches) -> Option<GlobalStateStrParams<'_>> {
    if let Some(state_root_hash) = state_root_hash::get(matches) {
        return Some(GlobalStateStrParams {
            str_identifier: GlobalStateStrIdentifier::Hash {
                is_block_hash: false,
            },
            identifier_value: state_root_hash,
        });
    }
    if let Some(block_hash) = block_hash::get(matches) {
        return Some(GlobalStateStrParams {
            str_identifier: GlobalStateStrIdentifier::Hash {
                is_block_hash: true,
            },
            identifier_value: block_hash,
        });
    }
    if let Some(block_height) = block_height::get(matches) {
        return Some(GlobalStateStrParams {
            str_identifier: GlobalStateStrIdentifier::Height,
            identifier_value: block_height,
        });
    }
    None
}

fn purse_str_params(matches: &ArgMatches) -> Result<PurseStrParams, CliError> {
    if let Ok(account_public_key) = public_key::get(matches) {
        return Ok(PurseStrParams {
            purse_str_identifier: PurseStrIdentifier::PublicKey,
            identifier_value: account_public_key,
        });
    }
    if let Some(formatted_account_hash) = account_hash::get(matches) {
        return Ok(PurseStrParams {
            purse_str_identifier: PurseStrIdentifier::AccountHash,
            identifier_value: formatted_account_hash.to_string(),
        });
    }
    if let Some(formatted_purse_uref) = common::purse_uref::get(matches) {
        return Ok(PurseStrParams {
            purse_str_identifier: PurseStrIdentifier::PurseURef,
            identifier_value: formatted_purse_uref.to_string(),
        });
    }
    unreachable!("clap arg groups and parsing should prevent this for purse identifier params")
}

#[async_trait]
impl ClientCommand for QueryBalance {
    const NAME: &'static str = "query-balance";
    const ABOUT: &'static str = "Retrieve a purse's balance from the network";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(block_hash::arg())
            .arg(block_height::arg())
            .arg(state_root_hash::arg())
            .group(
                ArgGroup::new("state-identifier")
                    .arg(block_hash::ARG_NAME)
                    .arg(block_height::ARG_NAME)
                    .arg(state_root_hash::ARG_NAME)
                    .required(false),
            )
            .arg(public_key::arg())
            .arg(account_hash::arg().required_unless_present(common::public_key::ARG_NAME))
            .arg(
                common::purse_uref::arg(DisplayOrder::PurseURef as usize, false)
                    .required_unless_present(common::public_key::ARG_NAME)
                    .required_unless_present(account_hash::ARG_NAME),
            )
            .group(
                ArgGroup::new("purse-identifier")
                    .arg(common::public_key::ARG_NAME)
                    .arg(account_hash::ARG_NAME)
                    .arg(common::purse_uref::ARG_NAME)
                    .required(true),
            )
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let maybe_global_state_str_params = maybe_global_state_str_params(matches);
        let purse_str_params = purse_str_params(matches)?;

        casper_client::cli::query_balance(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_global_state_str_params,
            purse_str_params,
        )
        .await
        .map(Success::from)
    }
}
