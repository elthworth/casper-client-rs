use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

pub struct GetBalance;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    StateRootHash,
    PurseURef,
}

#[async_trait]
impl ClientCommand for GetBalance {
    const NAME: &'static str = "get-balance";
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
            .arg(common::state_root_hash::arg(
                DisplayOrder::StateRootHash as usize,
                true,
            ))
            .arg(common::purse_uref::arg(
                DisplayOrder::PurseURef as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let state_root_hash = common::state_root_hash::get(matches)
            .unwrap_or_else(|| panic!("should have {} arg", common::state_root_hash::ARG_NAME));
        let purse_uref = common::purse_uref::get(matches)
            .unwrap_or_else(|| panic!("should have {} arg", common::purse_uref::ARG_NAME));

        casper_client::cli::get_balance(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            state_root_hash,
            purse_uref,
        )
        .await
        .map(Success::from)
    }
}
