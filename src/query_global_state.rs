use std::{fs, str};

use async_trait::async_trait;
use clap::{Arg, ArgGroup, ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

/// Legacy name of command.
const COMMAND_ALIAS: &str = "query-state";
/// Legacy name of block identifier argument.
const BLOCK_IDENTIFIER_ALIAS: &str = "block-hash";

pub struct QueryGlobalState;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier,
    StateRootHash,
    Key,
    Path,
}

/// Handles providing the arg for and retrieval of the key.
mod key {
    use casper_client::AsymmetricKeyExt;
    use casper_types::{AsymmetricType, PublicKey};

    use super::*;

    const ARG_NAME: &str = "key";
    const ARG_SHORT: char = 'k';
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "The base key for the query. This must be a properly formatted public key, account hash, \
        contract address hash, URef, transfer hash, deploy-info hash,era-info number, bid, withdraw \
        or dictionary address. The format for each respectively is \"<HEX STRING>\", \
        \"account-hash-<HEX STRING>\", \"hash-<HEX STRING>\", \
        \"uref-<HEX STRING>-<THREE DIGIT INTEGER>\", \"transfer-<HEX-STRING>\", \
        \"deploy-<HEX-STRING>\", \"era-<u64>\", \"bid-<HEX-STRING>\",\
        \"withdraw-<HEX-STRING>\" or \"dictionary-<HEX-STRING>\". \
        The system contract registry key is unique and can only take the value: \
        system-contract-registry-0000000000000000000000000000000000000000000000000000000000000000. \
        \nThe public key may instead be read in from a file, in which case \
        enter the path to the file as the --key argument. The file should be one of the two public \
        key files generated via the `keygen` subcommand; \"public_key_hex\" or \"public_key.pem\"";

    pub(crate) fn arg(order: usize) -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(order)
    }

    pub(crate) fn get(matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches
            .value_of(ARG_NAME)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME));

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
                    context: "query".to_string(),
                    error,
                }
            })?;
            return Ok(hex_public_key);
        }

        // Just return the value.
        Ok(value.to_string())
    }
}

/// Handles providing the arg for and retrieval of the key.
mod path {
    use super::*;

    const ARG_NAME: &str = "query-path";
    const ARG_SHORT: char = 'q';
    const ARG_VALUE_NAME: &str = "PATH/FROM/KEY";
    const ARG_HELP: &str = "The path from the key of the query";

    pub(crate) fn arg(order: usize) -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(order)
    }

    pub(crate) fn get(matches: &ArgMatches) -> &str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

#[async_trait]
impl ClientCommand for QueryGlobalState {
    const NAME: &'static str = "query-global-state";
    const ABOUT: &'static str = "Retrieve a stored value from the network";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .alias(COMMAND_ALIAS)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(
                common::block_identifier::arg(DisplayOrder::BlockIdentifier as usize, false)
                    .alias(BLOCK_IDENTIFIER_ALIAS),
            )
            .arg(common::state_root_hash::arg(
                DisplayOrder::StateRootHash as usize,
                false,
            ))
            .group(
                ArgGroup::new("state-identifier")
                    .arg(common::block_identifier::ARG_NAME)
                    .arg(common::state_root_hash::ARG_NAME)
                    .required(true),
            )
            .arg(key::arg(DisplayOrder::Key as usize))
            .arg(path::arg(DisplayOrder::Path as usize))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);

        let maybe_block_id = common::block_identifier::get(matches);
        let maybe_state_root_hash = common::state_root_hash::get(matches).unwrap_or_default();
        let key = key::get(matches)?;
        let path = path::get(matches);

        casper_client::cli::query_global_state(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
            maybe_state_root_hash,
            &key,
            path,
        )
        .await
        .map(Success::from)
    }
}
