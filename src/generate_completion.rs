use std::{fs::File, path::PathBuf, process};

use async_trait::async_trait;
use clap::{crate_name, value_parser, Arg, ArgMatches, Command};
use clap_complete::Shell;

use casper_client::{cli::CliError, Error};

use crate::{command::ClientCommand, common, Success};

/// This struct defines the order in which the args are shown for this subcommand's help message.
enum DisplayOrder {
    OutputDir,
    Force,
    Shell,
}

/// Handles providing the arg for and retrieval of the output directory.
mod output_dir {
    use super::*;

    const ARG_NAME: &str = "output-dir";
    const ARG_SHORT: char = 'o';
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str =
        "Path to output directory. If the path doesn't exist, the command will fail. Default path \
        normally requires running the command with sudo";
    const ARG_DEFAULT: &str = "/usr/share/bash-completion/completions";

    pub(super) fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .default_value(ARG_DEFAULT)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::OutputDir as usize)
    }

    pub(super) fn get(matches: &ArgMatches) -> PathBuf {
        matches
            .get_one::<String>(ARG_NAME)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
            .into()
    }
}

/// Handles providing the arg for and retrieval of shell type.
mod shell {
    use super::*;

    const ARG_NAME: &str = "shell";
    const ARG_VALUE_NAME: &str = common::ARG_STRING;
    const ARG_DEFAULT: &str = "bash";
    const ARG_HELP: &str = "The type of shell to generate the completion script for";

    pub fn arg() -> Arg<'static> {
        Arg::new(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .default_value(ARG_DEFAULT)
            .value_parser(value_parser!(Shell))
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Shell as usize)
    }

    pub fn get(matches: &ArgMatches) -> Shell {
        *matches
            .get_one::<Shell>(ARG_NAME)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

pub struct GenerateCompletion {}

#[async_trait]
impl ClientCommand for GenerateCompletion {
    const NAME: &'static str = "generate-completion";
    const ABOUT: &'static str = "Generate a shell completion script";

    fn build(display_order: usize) -> Command<'static> {
        Command::new(Self::NAME)
            .about(Self::ABOUT)
            .display_order(display_order)
            .arg(output_dir::arg())
            .arg(common::force::arg(DisplayOrder::Force as usize, true))
            .arg(shell::arg())
    }

    async fn run(matches: &ArgMatches) -> Result<Success, CliError> {
        let output_dir = output_dir::get(matches);
        let force = common::force::get(matches);
        let shell = shell::get(matches);
        let output_file_path = output_dir.join(crate_name!());

        if !force && output_file_path.exists() {
            eprintln!(
                "{} exists. To overwrite, rerun with --{}",
                output_file_path.display(),
                common::force::ARG_NAME
            );
            process::exit(1);
        }

        let mut output_file = File::create(&output_file_path).map_err(|error| {
            CliError::Core(Error::IoError {
                context: output_file_path.display().to_string(),
                error,
            })
        })?;

        clap_complete::generate(shell, &mut super::cli(), crate_name!(), &mut output_file);

        Ok(Success::Output(format!(
            "Wrote completion script for {} to {}",
            shell,
            output_dir.display()
        )))
    }
}
