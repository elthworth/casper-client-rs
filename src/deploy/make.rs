use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::{CliError, DeployStrParams};

use super::creation_common;
use crate::{command::ClientCommand, common, Success};

pub struct MakeDeploy;

#[async_trait]
impl ClientCommand for MakeDeploy {
    const NAME: &'static str = "make-deploy";
    const ABOUT: &'static str =
        "Creates a deploy and outputs it to a file or stdout. As a file, the deploy can \
        subsequently be signed by other parties using the 'sign-deploy' subcommand and then sent \
        to the network for execution using the 'send-deploy' subcommand";

    fn build(display_order: usize) -> Command<'static> {
        let subcommand = Command::new(Self::NAME)
            .about(Self::ABOUT)
            .arg(creation_common::output::arg())
            .arg(common::force::arg(
                creation_common::DisplayOrder::Force as usize,
                true,
            ))
            .display_order(display_order);
        let subcommand = creation_common::apply_common_session_options(subcommand);
        let subcommand = creation_common::apply_common_payment_options(subcommand);
        creation_common::apply_common_creation_options(subcommand, false)
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        creation_common::show_arg_examples_and_exit_if_required(matches);

        let secret_key = common::secret_key::get(matches);
        let timestamp = creation_common::timestamp::get(matches);
        let ttl = creation_common::ttl::get(matches);
        let chain_name = creation_common::chain_name::get(matches);

        let session_str_params = creation_common::session_str_params(matches);
        let payment_str_params = creation_common::payment_str_params(matches);

        let maybe_output_path = creation_common::output::get(matches).unwrap_or_default();
        let session_account = common::session_account::get(matches).unwrap_or_default();

        let force = common::force::get(matches);

        casper_client::cli::make_deploy(
            maybe_output_path,
            DeployStrParams {
                secret_key,
                timestamp,
                ttl,
                chain_name,
                session_account: &session_account,
            },
            session_str_params,
            payment_str_params,
            force,
        )
        .map(|_| {
            Success::Output(if maybe_output_path.is_empty() {
                String::new()
            } else {
                format!("Wrote the deploy to {}", maybe_output_path)
            })
        })
    }
}
