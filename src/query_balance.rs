use std::str;

use async_trait::async_trait;
use clap::{Arg, ArgGroup, ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

/// Legacy name of command.
const COMMAND_ALIAS: &str = "get-balance";
/// Legacy name of purse identifier argument from when the command was named "get-balance".
const PURSE_IDENTIFIER_ALIAS: &str = "purse-uref";

/// String to explain how to use the block identifier and state root hash args.
const AFTER_HELP: &str =
    "NOTE: The balance is retrieved as at a given state root hash specified by the \
    \"--block-identifier\" option or the \"--state-root-hash\" option. If neither is provided, the \
    state from the latest block known on the node will be used.";

pub struct QueryBalance;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier,
    StateRootHash,
    PurseIdentifier,
}

mod purse_identifier {
    use super::*;

    pub(super) const ARG_NAME: &str = "purse-identifier";
    const ARG_SHORT: char = 'p';
    const ARG_VALUE_NAME: &str = "FORMATTED STRING or PATH";
    const ARG_HELP: &str =
        "The identifier for the purse. This can be a public key or account hash, implying the main \
        purse of the given account should be used. Alternatively it can be a purse URef. To \
        provide a public key, it must be a properly formatted public key. The public key may \
        be read in from a file, in which case enter the path to the file as the --purse-identifier \
        argument. The file should be one of the two public key files generated via the `keygen` \
        subcommand; \"public_key_hex\" or \"public_key.pem\". To provide an account hash, it must \
        be formatted as \"account-hash-<HEX STRING>\", or for a URef as \
        \"uref-<HEX STRING>-<THREE DIGIT INTEGER>\"";

    pub fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .alias(PURSE_IDENTIFIER_ALIAS)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(true)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PurseIdentifier as usize)
    }

    pub fn get(matches: &ArgMatches) -> Result<String, CliError> {
        let value = matches.value_of(ARG_NAME).unwrap_or_default();
        common::public_key::try_read_from_file(value)
    }
}

#[async_trait]
impl ClientCommand for QueryBalance {
    const NAME: &'static str = "query-balance";
    const ABOUT: &'static str = "Retrieve a purse's balance from the network";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .alias(COMMAND_ALIAS)
            .about(Self::ABOUT)
            .after_help(AFTER_HELP)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockIdentifier as usize,
                false,
            ))
            .arg(common::state_root_hash::arg(
                DisplayOrder::StateRootHash as usize,
                false,
            ))
            .group(
                ArgGroup::new("state-identifier")
                    .arg(common::block_identifier::ARG_NAME)
                    .arg(common::state_root_hash::ARG_NAME)
                    .required(false),
            )
            .arg(purse_identifier::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);

        let maybe_block_id = common::block_identifier::get(matches);
        let maybe_state_root_hash = common::state_root_hash::get(matches).unwrap_or_default();
        let purse_id = purse_identifier::get(matches)?;

        casper_client::cli::query_balance(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
            maybe_state_root_hash,
            purse_id.as_str(),
        )
        .await
        .map(Success::from)
    }
}
