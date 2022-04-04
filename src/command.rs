use async_trait::async_trait;
use clap::{ArgMatches, Command};
use serde::Serialize;
use serde_json::Value;

use casper_client::cli::CliError;

/// The result of a successful execution of a given client command.
pub enum Success {
    /// The success response to a JSON-RPC request.
    Response(Value),
    /// The output which should be presented to the user for non-RPC client commands.
    Output(String),
}

impl<T: Serialize> From<T> for Success {
    fn from(response: T) -> Self {
        Success::Response(serde_json::to_value(response).expect("should JSON-encode response"))
    }
}

#[async_trait]
pub trait ClientCommand {
    const NAME: &'static str;
    const ABOUT: &'static str;
    /// Constructs the clap subcommand.
    fn build(display_order: usize) -> Command<'static>;

    /// Parses the arg matches and runs the subcommand.
    async fn run(matches: &ArgMatches) -> Result<Success, CliError>;
}
