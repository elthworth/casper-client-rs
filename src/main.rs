mod account_address;
mod block;
mod command;
mod common;
mod deploy;
mod docs;
mod generate_completion;
mod get_account_info;
mod get_auction_info;
mod get_balance;
mod get_dictionary_item;
mod get_era_info_by_switch_block;
mod get_state_hash;
mod get_validator_changes;
mod keygen;
mod query_global_state;

use std::process;

use clap::{crate_version, Command};

use casper_client::Error;
use casper_node::rpcs::{
    account::PutDeploy,
    chain::{GetBlock, GetBlockTransfers, GetEraInfoBySwitchBlock, GetStateRootHash},
    docs::ListRpcs,
    info::{GetDeploy, GetValidatorChanges},
    state::{GetAccountInfo, GetAuctionInfo, GetBalance, GetDictionaryItem, QueryGlobalState},
};

use account_address::GenerateAccountHash as AccountAddress;
use command::{ClientCommand, Success};
use deploy::{ListDeploys, MakeDeploy, MakeTransfer, SendDeploy, SignDeploy, Transfer};
use generate_completion::GenerateCompletion;
use keygen::Keygen;

const APP_NAME: &str = "Casper client";

/// This struct defines the order in which the subcommands are shown in the app's help message.
enum DisplayOrder {
    PutDeploy,
    MakeDeploy,
    SignDeploy,
    SendDeploy,
    Transfer,
    MakeTransfer,
    GetDeploy,
    GetBlock,
    GetBlockTransfers,
    ListDeploys,
    GetStateRootHash,
    QueryGlobalState,
    GetDictionaryItem,
    GetBalance,
    GetAccountInfo,
    GetEraInfo,
    GetAuctionInfo,
    GetValidatorChanges,
    Keygen,
    GenerateCompletion,
    GetRpcs,
    AccountAddress,
}

fn cli() -> Command<'static> {
    Command::new(APP_NAME)
        .version(crate_version!())
        .about("A client for interacting with the Casper network")
        .subcommand(PutDeploy::build(DisplayOrder::PutDeploy as usize))
        .subcommand(MakeDeploy::build(DisplayOrder::MakeDeploy as usize))
        .subcommand(SignDeploy::build(DisplayOrder::SignDeploy as usize))
        .subcommand(SendDeploy::build(DisplayOrder::SendDeploy as usize))
        .subcommand(Transfer::build(DisplayOrder::Transfer as usize))
        .subcommand(MakeTransfer::build(DisplayOrder::MakeTransfer as usize))
        .subcommand(GetDeploy::build(DisplayOrder::GetDeploy as usize))
        .subcommand(GetBlock::build(DisplayOrder::GetBlock as usize))
        .subcommand(GetBlockTransfers::build(
            DisplayOrder::GetBlockTransfers as usize,
        ))
        .subcommand(ListDeploys::build(DisplayOrder::ListDeploys as usize))
        .subcommand(GetBalance::build(DisplayOrder::GetBalance as usize))
        .subcommand(GetAccountInfo::build(DisplayOrder::GetAccountInfo as usize))
        .subcommand(GetStateRootHash::build(
            DisplayOrder::GetStateRootHash as usize,
        ))
        .subcommand(GetEraInfoBySwitchBlock::build(
            DisplayOrder::GetEraInfo as usize,
        ))
        .subcommand(GetAuctionInfo::build(DisplayOrder::GetAuctionInfo as usize))
        .subcommand(GetValidatorChanges::build(
            DisplayOrder::GetValidatorChanges as usize,
        ))
        .subcommand(Keygen::build(DisplayOrder::Keygen as usize))
        .subcommand(GenerateCompletion::build(
            DisplayOrder::GenerateCompletion as usize,
        ))
        .subcommand(ListRpcs::build(DisplayOrder::GetRpcs as usize))
        .subcommand(AccountAddress::build(DisplayOrder::AccountAddress as usize))
        .subcommand(GetDictionaryItem::build(
            DisplayOrder::GetDictionaryItem as usize,
        ))
        .subcommand(
            QueryGlobalState::build(DisplayOrder::QueryGlobalState as usize).alias("query-state"),
        )
}

#[tokio::main]
async fn main() {
    let arg_matches = cli().get_matches();
    let (result, matches) = match arg_matches.subcommand() {
        Some((PutDeploy::NAME, matches)) => (PutDeploy::run(matches).await, matches),
        Some((MakeDeploy::NAME, matches)) => (MakeDeploy::run(matches).await, matches),
        Some((SignDeploy::NAME, matches)) => (SignDeploy::run(matches).await, matches),
        Some((SendDeploy::NAME, matches)) => (SendDeploy::run(matches).await, matches),
        Some((Transfer::NAME, matches)) => (Transfer::run(matches).await, matches),
        Some((MakeTransfer::NAME, matches)) => (MakeTransfer::run(matches).await, matches),
        Some((GetDeploy::NAME, matches)) => (GetDeploy::run(matches).await, matches),
        Some((GetBlock::NAME, matches)) => (GetBlock::run(matches).await, matches),
        Some((GetBlockTransfers::NAME, matches)) => {
            (GetBlockTransfers::run(matches).await, matches)
        }
        Some((ListDeploys::NAME, matches)) => (ListDeploys::run(matches).await, matches),
        Some((GetBalance::NAME, matches)) => (GetBalance::run(matches).await, matches),
        Some((GetAccountInfo::NAME, matches)) => (GetAccountInfo::run(matches).await, matches),
        Some((GetStateRootHash::NAME, matches)) => (GetStateRootHash::run(matches).await, matches),
        Some((GetEraInfoBySwitchBlock::NAME, matches)) => {
            (GetEraInfoBySwitchBlock::run(matches).await, matches)
        }
        Some((GetAuctionInfo::NAME, matches)) => (GetAuctionInfo::run(matches).await, matches),
        Some((GetValidatorChanges::NAME, matches)) => {
            (GetValidatorChanges::run(matches).await, matches)
        }
        Some((Keygen::NAME, matches)) => (Keygen::run(matches).await, matches),
        Some((GenerateCompletion::NAME, matches)) => {
            (GenerateCompletion::run(matches).await, matches)
        }
        Some((ListRpcs::NAME, matches)) => (ListRpcs::run(matches).await, matches),
        Some((AccountAddress::NAME, matches)) => (AccountAddress::run(matches).await, matches),
        Some((GetDictionaryItem::NAME, matches)) => {
            (GetDictionaryItem::run(matches).await, matches)
        }
        Some((QueryGlobalState::NAME, matches)) => (QueryGlobalState::run(matches).await, matches),

        _ => {
            let _ = cli().print_long_help();
            println!();
            process::exit(1);
        }
    };

    let mut verbosity_level = common::verbose::get(matches);
    if verbosity_level == 0 {
        verbosity_level += 1
    }

    match &result {
        Ok(Success::Response(response)) => {
            casper_client::pretty_print_at_level(&response, verbosity_level)
        }
        Ok(Success::Output(output)) => println!("{}", output),
        Err(Error::ResponseIsError(error)) => {
            casper_client::pretty_print_at_level(&error, verbosity_level);
            process::exit(1);
        }
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    }
}
