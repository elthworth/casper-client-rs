use std::str;

use async_trait::async_trait;
use clap::{ArgGroup, ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

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

#[async_trait]
impl ClientCommand for QueryBalance {
    const NAME: &'static str = "query-balance";
    const ABOUT: &'static str = "Retrieve a purse's balance from the network";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
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
            .arg(common::purse_identifier::arg(
                DisplayOrder::PurseIdentifier as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);

        let maybe_block_id = common::block_identifier::get(matches);
        let maybe_state_root_hash = common::state_root_hash::get(matches).unwrap_or_default();
        let purse_id = common::purse_identifier::get(matches)?;

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
