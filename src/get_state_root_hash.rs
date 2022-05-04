use std::str;

use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use crate::{command::ClientCommand, common, Success};

pub struct GetStateRootHash;

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    Verbose,
    NodeAddress,
    RpcId,
    BlockHash,
}

#[async_trait]
impl ClientCommand for GetStateRootHash {
    const NAME: &'static str = "get-state-root-hash";
    const ABOUT: &'static str = "Retrieve a state root hash at a given block";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(common::verbose::arg(DisplayOrder::Verbose as usize))
            .arg(common::node_address::arg(
                DisplayOrder::NodeAddress as usize,
            ))
            .arg(common::rpc_id::arg(DisplayOrder::RpcId as usize))
            .arg(common::block_identifier::arg(
                DisplayOrder::BlockHash as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let maybe_rpc_id = common::rpc_id::get(matches);
        let node_address = common::node_address::get(matches);
        let verbosity_level = common::verbose::get(matches);
        let maybe_block_id = common::block_identifier::get(matches);

        casper_client::cli::get_state_root_hash(
            maybe_rpc_id,
            node_address,
            verbosity_level,
            maybe_block_id,
        )
        .await
        .map(Success::from)
    }
}
