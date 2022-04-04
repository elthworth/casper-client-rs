use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

const ALIAS: &str = "get-era-info-by-switch-block";

pub struct GetEraInfo;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockIdentifier,
}

#[async_trait]
impl ClientCommand for GetEraInfo {
    const NAME: &'static str = "get-era-info";
    const ABOUT: &'static str = "Retrieve era information from the network";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .alias(ALIAS)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockIdentifier as usize,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let maybe_block_id = common::block_identifier::get(matches);

        casper_client::cli::get_era_info(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
        )
        .await
        .map(Success::from)
    }
}
