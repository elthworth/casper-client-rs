use async_trait::async_trait;
use clap::{ArgMatches, Command};

use casper_client::cli::CliError;

use super::creation_common;
use crate::{command::ClientCommand, common, Success};

pub struct SignDeploy;

#[async_trait]
impl ClientCommand for SignDeploy {
    const NAME: &'static str = "sign-deploy";
    const ABOUT: &'static str =
        "Reads a previously-saved deploy from a file, cryptographically signs it, and outputs it \
        to a file or stdout";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(
                common::secret_key::arg(creation_common::DisplayOrder::SecretKey as usize)
                    .required(true),
            )
            .arg(creation_common::input::arg())
            .arg(creation_common::output::arg())
            .arg(common::force::arg(
                creation_common::DisplayOrder::Force as usize,
                true,
            ))
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let input_path = creation_common::input::get(matches);
        let secret_key = common::secret_key::get(matches);
        let maybe_output_path = creation_common::output::get(matches).unwrap_or_default();
        let force = common::force::get(matches);
        casper_client::cli::sign_deploy_file(input_path, secret_key, maybe_output_path, force).map(
            |_| {
                Success::Output(if maybe_output_path.is_empty() {
                    String::new()
                } else {
                    format!(
                        "Signed the deploy at {} and wrote to {}",
                        input_path, maybe_output_path
                    )
                })
            },
        )
    }
}
