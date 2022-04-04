use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

pub struct GetChainspec;

/// This enum defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
}

#[async_trait]
impl ClientCommand for GetChainspec {
    const NAME: &'static str = "get-chainspec";
    const ABOUT: &'static str =
        "Retrieve the chainspec of the network (to print the full TOML, run with '-vv')";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);

        casper_client::cli::get_chainspec(maybe_rpc_id, node_address, verbosity_level)
            .await
            .map(Success::from)
    }
}
