//! This module contains structs and helpers which are used by multiple subcommands related to
//! creating deploys.

use std::{convert::TryFrom, process};

use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
use lazy_static::lazy_static;

use casper_client::cl_type;
use casper_node::crypto::asymmetric_key::PublicKey as NodePublicKey;
use casper_types::{account::AccountHash, AccessRights, Key, URef};

use crate::common;

/// This struct defines the order in which the args are shown for this subcommand's help message.
pub(super) enum DisplayOrder {
    ShowArgExamples,
    Verbose,
    NodeAddress,
    RpcId,
    SecretKey,
    Input,
    Output,
    TransferAmount,
    TransferSourcePurse,
    TransferTargetAccount,
    TransferTargetPurse,
    Timestamp,
    Ttl,
    GasPrice,
    Dependencies,
    ChainName,
    SessionCode,
    SessionArgSimple,
    SessionArgsComplex,
    SessionHash,
    SessionName,
    SessionPackageHash,
    SessionPackageName,
    SessionEntryPoint,
    SessionVersion,
    StandardPayment,
    PaymentCode,
    PaymentArgSimple,
    PaymentArgsComplex,
    PaymentHash,
    PaymentName,
    PaymentPackageHash,
    PaymentPackageName,
    PaymentEntryPoint,
    PaymentVersion,
}

/// Handles providing the arg for and executing the show-arg-examples option.
pub(super) mod show_arg_examples {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "show-arg-examples";
    const ARG_SHORT: &str = "e";
    const ARG_HELP: &str =
        "If passed, all other options are ignored and a set of examples of session-/payment-args \
        is printed";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ShowArgExamples as usize)
    }

    pub(in crate::deploy) fn get(matches: &ArgMatches) -> bool {
        if !matches.is_present(ARG_NAME) {
            return false;
        }

        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();

        println!("Examples for passing values via --session-arg or --payment-arg:");
        println!("name_01:bool='false'");
        println!("name_02:i32='-1'");
        println!("name_03:i64='-2'");
        println!("name_04:u8='3'");
        println!("name_05:u32='4'");
        println!("name_06:u64='5'");
        println!("name_07:u128='6'");
        println!("name_08:u256='7'");
        println!("name_09:u512='8'");
        println!("name_10:unit=''");
        println!("name_11:string='a value'");
        println!(
            "key_account_name:key='{}'",
            Key::Account(AccountHash::new(array)).to_formatted_string()
        );
        println!(
            "key_hash_name:key='{}'",
            Key::Hash(array).to_formatted_string()
        );
        println!(
            "key_uref_name:key='{}'",
            Key::URef(URef::new(array, AccessRights::NONE)).to_formatted_string()
        );
        println!(
            "account_hash_name:account_hash='{}'",
            AccountHash::new(array).to_formatted_string()
        );
        println!(
            "uref_name:uref='{}'",
            URef::new(array, AccessRights::READ_ADD_WRITE).to_formatted_string()
        );
        println!(
            "public_key_name:public_key='{}'",
            NodePublicKey::from_hex(
                "0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1"
            )
            .unwrap()
            .to_hex()
        );

        true
    }
}

/// Handles providing the arg for and retrieval of the timestamp.
pub(super) mod timestamp {
    use super::*;

    const ARG_NAME: &str = "timestamp";
    const ARG_VALUE_NAME: &str = "MILLISECONDS";
    const ARG_HELP: &str =
        "Timestamp as the number of milliseconds since the Unix epoch. If not provided, the \
        current time will be used";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Timestamp as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the time to live.
pub(super) mod ttl {
    use super::*;

    const ARG_NAME: &str = "ttl";
    const ARG_VALUE_NAME: &str = "DURATION";
    const ARG_DEFAULT: &str = "1hour";
    const ARG_HELP: &str =
        "Time that the deploy will remain valid for. A deploy can only be included in a block \
        between `timestamp` and `timestamp + ttl`. Input examples: '1hr 12min', '30min 50sec', \
        '1day'. For all options, see \
        https://docs.rs/humantime/latest/humantime/fn.parse_duration.html";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Ttl as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the gas price.
pub(super) mod gas_price {
    use super::*;

    const ARG_NAME: &str = "gas-price";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_DEFAULT: &str = "10";
    const ARG_HELP: &str =
        "Conversion rate between the cost of Wasm opcodes and the motes sent by the payment code";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .default_value(ARG_DEFAULT)
            .help(ARG_HELP)
            .display_order(DisplayOrder::GasPrice as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the deploy dependencies.
pub(super) mod dependencies {
    use super::*;

    const ARG_NAME: &str = "dependency";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str =
        "A hex-encoded deploy hash of a deploy which must be executed before this deploy";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .multiple(true)
            .value_name(ARG_VALUE_NAME)
            .takes_value(true)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Dependencies as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
        matches
            .values_of(ARG_NAME)
            .iter()
            .map(|i| i.clone().map(|v| v))
            .flatten()
            .collect()
    }
}

/// Handles providing the arg for and retrieval of the chain name.
pub(super) mod chain_name {
    use super::*;

    const ARG_NAME: &str = "chain-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str =
        "Name of the chain, to avoid the deploy from being accidentally or maliciously included in \
        a different chain";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required_unless(show_arg_examples::ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::ChainName as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches
            .value_of(ARG_NAME)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

/// Handles providing the arg for and retrieval of the session code bytes.
pub(super) mod session_path {
    use super::*;

    pub(super) const ARG_NAME: &str = "session-path";
    const ARG_SHORT: &str = "s";
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to the compiled Wasm session code";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .short(ARG_SHORT)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::SessionCode as usize)
    }

    pub(in crate::deploy) fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of simple session and payment args.
pub(super) mod arg_simple {
    use super::*;

    const ARG_VALUE_NAME: &str = "NAME:TYPE='VALUE'";

    lazy_static! {
        static ref ARG_HELP: String = format!(
            "For simple CLTypes, a named and typed arg which is passed to the Wasm code. To see \
            an example for each type, run '--{}'. This arg can be repeated to pass multiple named, \
            typed args, but can only be used for the following types: {}",
            super::show_arg_examples::ARG_NAME,
            cl_type::supported_cl_type_list()
        );
    }

    pub(in crate::deploy) mod session {
        use super::*;

        pub const ARG_NAME: &str = "session-arg";
        const ARG_SHORT: &str = "a";

        pub fn arg() -> Arg<'static, 'static> {
            super::arg(ARG_NAME, DisplayOrder::SessionArgSimple as usize)
                .short(ARG_SHORT)
                .requires(super::session::ARG_NAME)
        }

        pub fn get<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
            matches
                .values_of(ARG_NAME)
                .iter()
                .map(|i| i.clone().map(|v| v))
                .flatten()
                .collect()
        }
    }

    pub(in crate::deploy) mod payment {
        use super::*;

        pub const ARG_NAME: &str = "payment-arg";

        pub fn arg() -> Arg<'static, 'static> {
            super::arg(ARG_NAME, DisplayOrder::PaymentArgSimple as usize)
                .requires(super::payment::ARG_NAME)
        }

        pub fn get<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
            matches
                .values_of(ARG_NAME)
                .iter()
                .map(|i| i.clone().map(|v| v))
                .flatten()
                .collect()
        }
    }

    fn arg(name: &'static str, order: usize) -> Arg<'static, 'static> {
        Arg::with_name(name)
            .long(name)
            .required(false)
            .multiple(true)
            .value_name(ARG_VALUE_NAME)
            .help(&*ARG_HELP)
            .display_order(order)
    }
}

/// Handles providing the arg for and retrieval of complex session and payment args.  These are read
/// in from a file.
pub(super) mod args_complex {
    use super::*;

    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str =
        "Path to file containing named and typed args for passing to the Wasm code";

    pub(in crate::deploy) mod session {
        use super::*;

        pub const ARG_NAME: &str = "session-args-complex";

        pub fn arg() -> Arg<'static, 'static> {
            super::arg(ARG_NAME, DisplayOrder::SessionArgsComplex as usize)
                .requires(super::session::ARG_NAME)
        }

        pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
            matches.value_of(ARG_NAME).unwrap_or_default()
        }
    }

    pub(in crate::deploy) mod payment {
        use super::*;

        pub const ARG_NAME: &str = "payment-args-complex";

        pub fn arg() -> Arg<'static, 'static> {
            super::arg(ARG_NAME, DisplayOrder::PaymentArgsComplex as usize)
                .requires(super::payment::ARG_NAME)
        }

        pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
            matches.value_of(ARG_NAME).unwrap_or_default()
        }
    }

    fn arg(name: &'static str, order: usize) -> Arg<'static, 'static> {
        Arg::with_name(name)
            .long(name)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(order)
    }
}

/// Handles providing the arg for and retrieval of the payment code bytes.
pub(super) mod payment_path {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "payment-path";
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to the compiled Wasm payment code";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::PaymentCode as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

/// Handles providing the arg for and retrieval of the payment-amount arg.
pub(super) mod standard_payment_amount {
    use super::*;

    pub(in crate::deploy) const ARG_NAME: &str = "payment-amount";
    const ARG_VALUE_NAME: &str = "AMOUNT";
    const ARG_SHORT: &str = "p";
    const ARG_HELP: &str =
        "If provided, uses the standard-payment system contract rather than custom payment Wasm. \
        The value is the 'amount' arg of the standard-payment contract. This arg is incompatible \
        with all other --payment-xxx args";

    pub(in crate::deploy) fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT)
            .required(false)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::StandardPayment as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) fn apply_common_creation_options<'a, 'b>(
    subcommand: App<'a, 'b>,
    include_node_address: bool,
) -> App<'a, 'b> {
    let mut subcommand = subcommand
        .setting(AppSettings::NextLineHelp)
        .arg(show_arg_examples::arg());

    if include_node_address {
        subcommand = subcommand.arg(
            common::node_address::arg(DisplayOrder::NodeAddress as usize)
                .required_unless(show_arg_examples::ARG_NAME),
        );
    }

    subcommand = subcommand
        .arg(
            common::secret_key::arg(DisplayOrder::SecretKey as usize)
                .required_unless(show_arg_examples::ARG_NAME),
        )
        .arg(timestamp::arg())
        .arg(ttl::arg())
        .arg(gas_price::arg())
        .arg(dependencies::arg())
        .arg(chain_name::arg());
    subcommand
}

pub(super) fn apply_common_session_options<'a, 'b>(subcommand: App<'a, 'b>) -> App<'a, 'b> {
    subcommand
        .arg(session_path::arg())
        .arg(session_package_hash::arg())
        .arg(session_package_name::arg())
        .arg(session_hash::arg())
        .arg(session_name::arg())
        .arg(arg_simple::session::arg())
        .arg(args_complex::session::arg())
        // Group the session-arg args so only one style is used to ensure consistent ordering.
        .group(
            ArgGroup::with_name("session-args")
                .arg(arg_simple::session::ARG_NAME)
                .arg(args_complex::session::ARG_NAME)
                .required(false),
        )
        .arg(session_entry_point::arg())
        .arg(session_version::arg())
        .group(
            ArgGroup::with_name("session")
                .arg(session_path::ARG_NAME)
                .arg(session_package_hash::ARG_NAME)
                .arg(session_package_name::ARG_NAME)
                .arg(session_hash::ARG_NAME)
                .arg(session_name::ARG_NAME)
                .arg(show_arg_examples::ARG_NAME)
                .required(true),
        )
}

pub(crate) fn apply_common_payment_options(
    subcommand: App<'static, 'static>,
) -> App<'static, 'static> {
    subcommand
        .arg(standard_payment_amount::arg())
        .arg(payment_path::arg())
        .arg(payment_package_hash::arg())
        .arg(payment_package_name::arg())
        .arg(payment_hash::arg())
        .arg(payment_name::arg())
        .arg(arg_simple::payment::arg())
        .arg(args_complex::payment::arg())
        // Group the payment-arg args so only one style is used to ensure consistent ordering.
        .group(
            ArgGroup::with_name("payment-args")
                .arg(arg_simple::payment::ARG_NAME)
                .arg(args_complex::payment::ARG_NAME)
                .required(false),
        )
        .arg(payment_entry_point::arg())
        .arg(payment_version::arg())
        .group(
            ArgGroup::with_name("payment")
                .arg(standard_payment_amount::ARG_NAME)
                .arg(payment_path::ARG_NAME)
                .arg(payment_package_hash::ARG_NAME)
                .arg(payment_package_name::ARG_NAME)
                .arg(payment_hash::ARG_NAME)
                .arg(payment_name::ARG_NAME)
                .arg(show_arg_examples::ARG_NAME)
                .required(true),
        )
}

pub(super) fn show_arg_examples_and_exit_if_required(matches: &ArgMatches<'_>) {
    // If we printed the arg examples, exit the process.
    if show_arg_examples::get(matches) {
        process::exit(0);
    }
}

pub(super) mod output {
    use super::*;

    const ARG_NAME: &str = "output";
    const ARG_SHORT_NAME: &str = "o";
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to output deploy file. If omitted, defaults to stdout. If file exists, it will be overwritten";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .required(false)
            .long(ARG_NAME)
            .short(ARG_SHORT_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Output as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod input {
    use super::*;

    const ARG_NAME: &str = "input";
    const ARG_SHORT_NAME: &str = "i";
    const ARG_VALUE_NAME: &str = common::ARG_PATH;
    const ARG_HELP: &str = "Path to input deploy file";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .required_unless(show_arg_examples::ARG_NAME)
            .long(ARG_NAME)
            .short(ARG_SHORT_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .display_order(DisplayOrder::Input as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches
            .value_of(ARG_NAME)
            .unwrap_or_else(|| panic!("should have {} arg", ARG_NAME))
    }
}

pub(super) mod session_hash {
    use super::*;

    pub const ARG_NAME: &str = "session-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored contract to be called as the session";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionHash as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod session_name {
    use super::*;

    pub const ARG_NAME: &str = "session-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored contract (associated with the executing account) to be called as the session";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionName as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod session_package_hash {
    use super::*;

    pub const ARG_NAME: &str = "session-package-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored package to be called as the session";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionPackageHash as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod session_package_name {
    use super::*;

    pub const ARG_NAME: &str = "session-package-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored package to be called as the session";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(session_entry_point::ARG_NAME)
            .display_order(DisplayOrder::SessionPackageName as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod session_entry_point {
    use super::*;

    pub const ARG_NAME: &str = "session-entry-point";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the method that will be used when calling the session contract";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::SessionEntryPoint as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod session_version {
    use super::*;

    pub const ARG_NAME: &str = "session-version";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "Version of the called session contract. Latest will be used by default";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::SessionVersion as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_hash {
    use super::*;

    pub const ARG_NAME: &str = "payment-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored contract to be called as the payment";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentHash as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_name {
    use super::*;

    pub const ARG_NAME: &str = "payment-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored contract (associated with the executing account) \
    to be called as the payment";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentName as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_package_hash {
    use super::*;

    pub const ARG_NAME: &str = "payment-package-hash";
    const ARG_VALUE_NAME: &str = common::ARG_HEX_STRING;
    const ARG_HELP: &str = "Hex-encoded hash of the stored package to be called as the payment";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentPackageHash as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_package_name {
    use super::*;

    pub const ARG_NAME: &str = "payment-package-name";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the stored package to be called as the payment";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .requires(payment_entry_point::ARG_NAME)
            .display_order(DisplayOrder::PaymentPackageName as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_entry_point {
    use super::*;

    pub const ARG_NAME: &str = "payment-entry-point";
    const ARG_VALUE_NAME: &str = "NAME";
    const ARG_HELP: &str = "Name of the method that will be used when calling the payment contract";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::PaymentEntryPoint as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}

pub(super) mod payment_version {
    use super::*;

    pub const ARG_NAME: &str = "payment-version";
    const ARG_VALUE_NAME: &str = common::ARG_INTEGER;
    const ARG_HELP: &str = "Version of the called payment contract. Latest will be used by default";

    pub fn arg() -> Arg<'static, 'static> {
        Arg::with_name(ARG_NAME)
            .long(ARG_NAME)
            .value_name(ARG_VALUE_NAME)
            .help(ARG_HELP)
            .required(false)
            .display_order(DisplayOrder::PaymentVersion as usize)
    }

    pub fn get<'a>(matches: &'a ArgMatches) -> &'a str {
        matches.value_of(ARG_NAME).unwrap_or_default()
    }
}