//! This module contains structs and helpers which are used by multiple subcommands related to
//! creating deploys.

use std::{convert::TryInto, fs, io, path::Path, str::FromStr};

use rand::Rng;
use serde::{self, Deserialize};

use casper_hashing::Digest;
use casper_types::{
    account::AccountHash, bytesrepr, AsymmetricType, CLType, CLValue, HashAddr, Key, NamedArg,
    PublicKey, RuntimeArgs, SecretKey, UIntParseError, URef, U512,
};

use super::{cl_type, CliError, PaymentStrParams, SessionStrParams};
use crate::{
    crypto::AsymmetricKeyExt,
    types::{BlockHash, DeployHash, ExecutableDeployItem, TimeDiff, Timestamp},
    BlockIdentifier, GlobalStateIdentifier, JsonRpcId, OutputKind, PurseIdentifier, Verbosity,
};

pub(super) fn rpc_id(maybe_rpc_id: &str) -> JsonRpcId {
    if maybe_rpc_id.is_empty() {
        JsonRpcId::from(rand::thread_rng().gen::<i64>())
    } else if let Ok(i64_id) = maybe_rpc_id.parse::<i64>() {
        JsonRpcId::from(i64_id)
    } else {
        JsonRpcId::from(maybe_rpc_id.to_string())
    }
}

pub(super) fn verbosity(verbosity_level: u64) -> Verbosity {
    match verbosity_level {
        0 => Verbosity::Low,
        1 => Verbosity::Medium,
        _ => Verbosity::High,
    }
}

pub(super) fn output_kind(maybe_output_path: &str, force: bool) -> OutputKind {
    if maybe_output_path.is_empty() {
        OutputKind::Stdout
    } else {
        OutputKind::file(Path::new(maybe_output_path), force)
    }
}

pub(super) fn secret_key_from_file<P: AsRef<Path>>(
    secret_key_path: P,
) -> Result<SecretKey, CliError> {
    SecretKey::from_file(secret_key_path).map_err(|error| {
        CliError::Core(crate::Error::CryptoError {
            context: "secret key",
            error,
        })
    })
}

pub(super) fn timestamp(value: &str) -> Result<Timestamp, CliError> {
    if value.is_empty() {
        return Ok(Timestamp::now());
    }
    Timestamp::from_str(value).map_err(|error| CliError::FailedToParseTimestamp {
        context: "timestamp",
        error,
    })
}

pub(super) fn ttl(value: &str) -> Result<TimeDiff, CliError> {
    TimeDiff::from_str(value).map_err(|error| CliError::FailedToParseTimeDiff {
        context: "ttl",
        error,
    })
}

pub(super) fn session_account(value: &str) -> Result<Option<PublicKey>, CliError> {
    if value.is_empty() {
        return Ok(None);
    }

    let public_key = PublicKey::from_hex(value).map_err(|error| crate::Error::CryptoError {
        context: "session account",
        error: crate::CryptoError::from(error),
    })?;
    Ok(Some(public_key))
}

/// Handles providing the arg for and retrieval of simple session and payment args.
mod arg_simple {
    use super::*;

    const ARG_VALUE_NAME: &str = r#""NAME:TYPE='VALUE'" OR "NAME:TYPE=null""#;

    pub(crate) mod session {
        use super::*;

        pub fn parse(values: &[&str]) -> Result<Option<RuntimeArgs>, CliError> {
            Ok(if values.is_empty() {
                None
            } else {
                Some(get(values)?)
            })
        }
    }

    pub(crate) mod payment {
        use super::*;

        pub fn parse(values: &[&str]) -> Result<Option<RuntimeArgs>, CliError> {
            Ok(if values.is_empty() {
                None
            } else {
                Some(get(values)?)
            })
        }
    }

    fn get(values: &[&str]) -> Result<RuntimeArgs, CliError> {
        let mut runtime_args = RuntimeArgs::new();
        for arg in values {
            let parts = split_arg(arg)?;
            parts_to_cl_value(parts, &mut runtime_args)?;
        }
        Ok(runtime_args)
    }

    /// Splits a single arg of the form `NAME:TYPE='VALUE'` into its constituent parts.
    fn split_arg(arg: &str) -> Result<(&str, CLType, &str), CliError> {
        let parts: Vec<_> = arg.splitn(3, &[':', '='][..]).collect();
        if parts.len() != 3 {
            return Err(CliError::InvalidCLValue(format!(
                "arg {} should be formatted as {}",
                arg, ARG_VALUE_NAME
            )));
        }
        let cl_type = cl_type::parse(parts[1]).map_err(|_| {
            CliError::InvalidCLValue(format!(
                "unknown variant {}, expected one of {}",
                parts[1],
                cl_type::help::supported_cl_type_list()
            ))
        })?;
        Ok((parts[0], cl_type, parts[2]))
    }

    /// Insert a value built from a single arg which has been split into its constituent parts.
    fn parts_to_cl_value(
        parts: (&str, CLType, &str),
        runtime_args: &mut RuntimeArgs,
    ) -> Result<(), CliError> {
        let (name, cl_type, value) = parts;
        let cl_value = cl_type::parts_to_cl_value(cl_type, value)?;
        runtime_args.insert_cl_value(name, cl_value);
        Ok(())
    }
}

/// Handles providing the arg for and retrieval of complex session and payment args. These are read
/// in from a file.
mod args_complex {
    use std::{
        fmt::{self, Formatter},
        result::Result as StdResult,
    };

    use serde::de::{Deserializer, Error as SerdeError, Visitor};

    use casper_types::checksummed_hex;

    use super::*;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum DeployArgValue {
        /// Contains `CLValue` serialized into bytes in base16 form.
        #[serde(deserialize_with = "deserialize_raw_bytes")]
        RawBytes(Vec<u8>),
    }

    fn deserialize_raw_bytes<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> StdResult<Vec<u8>, D::Error> {
        struct HexStrVisitor;

        impl<'de> Visitor<'de> for HexStrVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "a hex encoded string")
            }

            fn visit_str<E: SerdeError>(
                self,
                hex_encoded_input: &str,
            ) -> StdResult<Self::Value, E> {
                checksummed_hex::decode(hex_encoded_input).map_err(SerdeError::custom)
            }

            fn visit_borrowed_str<E: SerdeError>(
                self,
                hex_encoded_input: &'de str,
            ) -> StdResult<Self::Value, E> {
                checksummed_hex::decode(hex_encoded_input).map_err(SerdeError::custom)
            }
        }

        deserializer.deserialize_str(HexStrVisitor)
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    struct DeployArg {
        /// Deploy argument's name.
        name: String,
        value: DeployArgValue,
    }

    impl From<DeployArgValue> for CLValue {
        fn from(value: DeployArgValue) -> Self {
            match value {
                DeployArgValue::RawBytes(bytes) => bytesrepr::deserialize(bytes)
                    .unwrap_or_else(|error| panic!("should deserialize deploy arg: {}", error)),
            }
        }
    }

    impl From<DeployArg> for NamedArg {
        fn from(deploy_arg: DeployArg) -> Self {
            let cl_value = deploy_arg
                .value
                .try_into()
                .unwrap_or_else(|error| panic!("should serialize deploy arg: {}", error));
            NamedArg::new(deploy_arg.name, cl_value)
        }
    }

    pub mod session {
        use super::*;

        pub fn parse(path: &str) -> Result<Option<RuntimeArgs>, CliError> {
            if path.is_empty() {
                return Ok(None);
            }
            let runtime_args = get(path).map_err(|error| crate::Error::IoError {
                context: format!("error reading session file at '{}'", path),
                error,
            })?;
            Ok(Some(runtime_args))
        }
    }

    pub mod payment {
        use super::*;

        pub fn parse(path: &str) -> Result<Option<RuntimeArgs>, CliError> {
            if path.is_empty() {
                return Ok(None);
            }
            let runtime_args = get(path).map_err(|error| crate::Error::IoError {
                context: format!("error reading payment file at '{}'", path),
                error,
            })?;
            Ok(Some(runtime_args))
        }
    }

    fn get(path: &str) -> io::Result<RuntimeArgs> {
        let bytes = fs::read(path)?;
        // Received structured args in JSON format.
        let args: Vec<DeployArg> = serde_json::from_slice(&bytes)?;
        // Convert JSON deploy args into vector of named args.
        let mut named_args = Vec::with_capacity(args.len());
        for arg in args {
            named_args.push(arg.into());
        }
        Ok(RuntimeArgs::from(named_args))
    }
}

const STANDARD_PAYMENT_ARG_NAME: &str = "amount";
fn standard_payment(value: &str) -> Result<RuntimeArgs, CliError> {
    if value.is_empty() {
        return Err(CliError::InvalidCLValue(value.to_string()));
    }
    let arg = U512::from_dec_str(value).map_err(|err| CliError::FailedToParseUint {
        context: "amount",
        error: UIntParseError::FromDecStr(err),
    })?;
    let mut runtime_args = RuntimeArgs::new();
    runtime_args.insert(STANDARD_PAYMENT_ARG_NAME, arg)?;
    Ok(runtime_args)
}

fn args_from_simple_or_complex(
    simple: Option<RuntimeArgs>,
    complex: Option<RuntimeArgs>,
) -> RuntimeArgs {
    // We can have exactly zero or one of the two as `Some`.
    match (simple, complex) {
        (Some(args), None) | (None, Some(args)) => args,
        (None, None) => RuntimeArgs::new(),
        (Some(_), Some(_)) => unreachable!("should not have both simple and complex args"),
    }
}

/// Private macro for enforcing parameter validity.
/// e.g. check_exactly_one_not_empty!(
///   (field1) requires[another_field],
///   (field2) requires[another_field, yet_another_field]
///   (field3) requires[]
/// )
/// Returns an error if:
/// - More than one parameter is non-empty.
/// - Any parameter that is non-empty has requires[] requirements that are empty.
macro_rules! check_exactly_one_not_empty {
    ( context: $site:expr, $( ($x:expr) requires[$($y:expr),*] requires_empty[$($z:expr),*] ),+ $(,)? ) => {{

        let field_is_empty_map = &[$(
            (stringify!($x), $x.is_empty())
        ),+];

        let required_arguments = field_is_empty_map
            .iter()
            .filter(|(_, is_empty)| !*is_empty)
            .map(|(field, _)| field.to_string())
            .collect::<Vec<_>>();

        if required_arguments.is_empty() {
            let required_param_names = vec![$((stringify!($x))),+];
            return Err(CliError::InvalidArgument {
                context: $site,
                error: format!("Missing a required arg - exactly one of the following must be provided: {:?}", required_param_names),
            });
        }
        if required_arguments.len() == 1 {
            let name = &required_arguments[0];
            let field_requirements = &[$(
                (
                    stringify!($x),
                    $x,
                    vec![$((stringify!($y), $y)),*],
                    vec![$((stringify!($z), $z)),*],
                )
            ),+];

            // Check requires[] and requires_requires_empty[] fields
            let (_, value, requirements, required_empty) = field_requirements
                .iter()
                .find(|(field, _, _, _)| *field == name).expect("should exist");
            let required_arguments = requirements
                .iter()
                .filter(|(_, value)| !value.is_empty())
                .collect::<Vec<_>>();

            if requirements.len() != required_arguments.len() {
                let required_param_names = requirements
                    .iter()
                    .map(|(requirement_name, _)| requirement_name)
                    .collect::<Vec<_>>();
                return Err(CliError::InvalidArgument {
                    context: $site,
                    error: format!("Field {} also requires following fields to be provided: {:?}", name, required_param_names),
                });
            }

            let mut conflicting_fields = required_empty
                .iter()
                .filter(|(_, value)| !value.is_empty())
                .map(|(field, value)| format!("{}={}", field, value)).collect::<Vec<_>>();

            if !conflicting_fields.is_empty() {
                conflicting_fields.push(format!("{}={}", name, value));
                conflicting_fields.sort();
                return Err(CliError::ConflictingArguments{
                    context: $site,
                    args: conflicting_fields,
                });
            }
        } else {
            let mut non_empty_fields_with_values = vec![$((stringify!($x), $x)),+]
                .iter()
                .filter_map(|(field, value)| if !value.is_empty() {
                    Some(format!("{}={}", field, value))
                } else {
                    None
                })
                .collect::<Vec<String>>();
            non_empty_fields_with_values.sort();
            return Err(CliError::ConflictingArguments {
                context: $site,
                args: non_empty_fields_with_values,
            });
        }
    }}
}

pub(super) fn session_executable_deploy_item(
    params: SessionStrParams,
) -> Result<ExecutableDeployItem, CliError> {
    let SessionStrParams {
        session_hash,
        session_name,
        session_package_hash,
        session_package_name,
        session_path,
        ref session_args_simple,
        session_args_complex,
        session_version,
        session_entry_point,
        is_session_transfer: session_transfer,
    } = params;
    // This is to make sure that we're using &str consistently in the macro call below.
    let is_session_transfer = if session_transfer { "true" } else { "" };

    check_exactly_one_not_empty!(
        context: "parse_session_info",
        (session_hash)
            requires[session_entry_point] requires_empty[session_version],
        (session_name)
            requires[session_entry_point] requires_empty[session_version],
        (session_package_hash)
            requires[session_entry_point] requires_empty[],
        (session_package_name)
            requires[session_entry_point] requires_empty[],
        (session_path)
            requires[] requires_empty[session_entry_point, session_version],
        (is_session_transfer)
            requires[] requires_empty[session_entry_point, session_version]
    );
    if !session_args_simple.is_empty() && !session_args_complex.is_empty() {
        return Err(CliError::ConflictingArguments {
            context: "parse_session_info",
            args: vec!["session_args".to_owned(), "session_args_complex".to_owned()],
        });
    }

    let session_args = args_from_simple_or_complex(
        arg_simple::session::parse(session_args_simple)?,
        args_complex::session::parse(session_args_complex)?,
    );
    if session_transfer {
        if session_args.is_empty() {
            return Err(CliError::InvalidArgument {
                context: "is_session_transfer",
                error: "requires --session-arg to be present".to_string(),
            });
        }
        return Ok(ExecutableDeployItem::Transfer { args: session_args });
    }
    let invalid_entry_point = || CliError::InvalidArgument {
        context: "session_entry_point",
        error: session_entry_point.to_string(),
    };
    if let Some(session_name) = name(session_name) {
        return Ok(ExecutableDeployItem::StoredContractByName {
            name: session_name,
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    if let Some(session_hash) = contract_hash(session_hash)? {
        return Ok(ExecutableDeployItem::StoredContractByHash {
            hash: session_hash.into(),
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    let version = version(session_version).ok();
    if let Some(package_name) = name(session_package_name) {
        return Ok(ExecutableDeployItem::StoredVersionedContractByName {
            name: package_name,
            version, // defaults to highest enabled version
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    if let Some(package_hash) = contract_hash(session_package_hash)? {
        return Ok(ExecutableDeployItem::StoredVersionedContractByHash {
            hash: package_hash.into(),
            version, // defaults to highest enabled version
            entry_point: entry_point(session_entry_point).ok_or_else(invalid_entry_point)?,
            args: session_args,
        });
    }

    let module_bytes = fs::read(session_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read session file at '{}'", session_path),
        error,
    })?;
    Ok(ExecutableDeployItem::ModuleBytes {
        module_bytes: module_bytes.into(),
        args: session_args,
    })
}

pub(super) fn payment_executable_deploy_item(
    params: PaymentStrParams,
) -> Result<ExecutableDeployItem, CliError> {
    let PaymentStrParams {
        payment_amount,
        payment_hash,
        payment_name,
        payment_package_hash,
        payment_package_name,
        payment_path,
        ref payment_args_simple,
        payment_args_complex,
        payment_version,
        payment_entry_point,
    } = params;
    check_exactly_one_not_empty!(
        context: "parse_payment_info",
        (payment_amount)
            requires[] requires_empty[payment_entry_point, payment_version],
        (payment_hash)
            requires[payment_entry_point] requires_empty[payment_version],
        (payment_name)
            requires[payment_entry_point] requires_empty[payment_version],
        (payment_package_hash)
            requires[payment_entry_point] requires_empty[],
        (payment_package_name)
            requires[payment_entry_point] requires_empty[],
        (payment_path) requires[] requires_empty[payment_entry_point, payment_version],
    );
    if !payment_args_simple.is_empty() && !payment_args_complex.is_empty() {
        return Err(CliError::ConflictingArguments {
            context: "parse_payment_info",
            args: vec![
                "payment_args_simple".to_owned(),
                "payment_args_complex".to_owned(),
            ],
        });
    }

    if let Ok(payment_args) = standard_payment(payment_amount) {
        return Ok(ExecutableDeployItem::ModuleBytes {
            module_bytes: vec![].into(),
            args: payment_args,
        });
    }

    let invalid_entry_point = || CliError::InvalidArgument {
        context: "payment_entry_point",
        error: payment_entry_point.to_string(),
    };

    let payment_args = args_from_simple_or_complex(
        arg_simple::payment::parse(payment_args_simple)?,
        args_complex::payment::parse(payment_args_complex)?,
    );

    if let Some(payment_name) = name(payment_name) {
        return Ok(ExecutableDeployItem::StoredContractByName {
            name: payment_name,
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    if let Some(payment_hash) = contract_hash(payment_hash)? {
        return Ok(ExecutableDeployItem::StoredContractByHash {
            hash: payment_hash.into(),
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    let version = version(payment_version).ok();
    if let Some(package_name) = name(payment_package_name) {
        return Ok(ExecutableDeployItem::StoredVersionedContractByName {
            name: package_name,
            version, // defaults to highest enabled version
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    if let Some(package_hash) = contract_hash(payment_package_hash)? {
        return Ok(ExecutableDeployItem::StoredVersionedContractByHash {
            hash: package_hash.into(),
            version, // defaults to highest enabled version
            entry_point: entry_point(payment_entry_point).ok_or_else(invalid_entry_point)?,
            args: payment_args,
        });
    }

    let module_bytes = fs::read(payment_path).map_err(|error| crate::Error::IoError {
        context: format!("unable to read payment file at '{}'", payment_path),
        error,
    })?;
    Ok(ExecutableDeployItem::ModuleBytes {
        module_bytes: module_bytes.into(),
        args: payment_args,
    })
}

fn contract_hash(value: &str) -> Result<Option<HashAddr>, CliError> {
    if value.is_empty() {
        return Ok(None);
    }
    match Digest::from_hex(value) {
        Ok(digest) => Ok(Some(digest.value())),
        Err(error) => {
            if let Ok(Key::Hash(hash)) = Key::from_formatted_str(value) {
                Ok(Some(hash))
            } else {
                Err(CliError::FailedToParseDigest {
                    context: "contract hash",
                    error,
                })
            }
        }
    }
}

fn name(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn entry_point(value: &str) -> Option<String> {
    if value.is_empty() {
        return None;
    }
    Some(value.to_string())
}

fn version(value: &str) -> Result<u32, CliError> {
    value
        .parse::<u32>()
        .map_err(|error| CliError::FailedToParseInt {
            context: "version",
            error,
        })
}

pub(super) fn transfer_id(value: &str) -> Result<u64, CliError> {
    value.parse().map_err(|error| CliError::FailedToParseInt {
        context: "transfer-id",
        error,
    })
}

pub(super) fn block_identifier(
    maybe_block_identifier: &str,
) -> Result<Option<BlockIdentifier>, CliError> {
    if maybe_block_identifier.is_empty() {
        return Ok(None);
    }

    if maybe_block_identifier.len() == (Digest::LENGTH * 2) {
        let hash = Digest::from_hex(maybe_block_identifier).map_err(|error| {
            CliError::FailedToParseDigest {
                context: "block_identifier",
                error,
            }
        })?;
        Ok(Some(BlockIdentifier::Hash(BlockHash::new(hash))))
    } else {
        let height =
            maybe_block_identifier
                .parse()
                .map_err(|error| CliError::FailedToParseInt {
                    context: "block_identifier",
                    error,
                })?;
        Ok(Some(BlockIdentifier::Height(height)))
    }
}

pub(super) fn deploy_hash(deploy_hash: &str) -> Result<DeployHash, CliError> {
    let hash = Digest::from_hex(deploy_hash).map_err(|error| CliError::FailedToParseDigest {
        context: "deploy hash",
        error,
    })?;
    Ok(DeployHash::new(hash))
}

pub(super) fn key_for_query(key: &str) -> Result<Key, CliError> {
    match Key::from_formatted_str(key) {
        Ok(key) => Ok(key),
        Err(error) => {
            if let Ok(public_key) = PublicKey::from_hex(key) {
                Ok(Key::Account(public_key.to_account_hash()))
            } else {
                Err(CliError::FailedToParseKey {
                    context: "key for query",
                    error,
                })
            }
        }
    }
}

/// `maybe_block_id` can be either a block hash or a block height.
pub(super) fn global_state_identifier(
    maybe_block_id: &str,
    maybe_state_root_hash: &str,
) -> Result<Option<GlobalStateIdentifier>, CliError> {
    match block_identifier(maybe_block_id)? {
        Some(BlockIdentifier::Hash(hash)) => {
            return Ok(Some(GlobalStateIdentifier::BlockHash(hash)))
        }
        Some(BlockIdentifier::Height(height)) => {
            return Ok(Some(GlobalStateIdentifier::BlockHeight(height)))
        }
        None => (),
    }

    if maybe_state_root_hash.is_empty() {
        return Ok(None);
    }

    let state_root_hash =
        Digest::from_hex(maybe_state_root_hash).map_err(|error| CliError::FailedToParseDigest {
            context: "state root hash in global_state_identifier",
            error,
        })?;
    Ok(Some(GlobalStateIdentifier::StateRootHash(state_root_hash)))
}

/// `purse_id` can be a formatted public key, account hash, or URef.  It may not be empty.
pub(super) fn purse_identifier(purse_id: &str) -> Result<PurseIdentifier, CliError> {
    const ACCOUNT_HASH_PREFIX: &str = "account-hash-";
    const UREF_PREFIX: &str = "uref-";

    if purse_id.is_empty() {
        return Err(CliError::InvalidArgument {
            context: "purse_identifier",
            error: "cannot be empty string".to_string(),
        });
    }

    if purse_id.starts_with(ACCOUNT_HASH_PREFIX) {
        let account_hash = AccountHash::from_formatted_str(purse_id).map_err(|error| {
            CliError::FailedToParseAccountHash {
                context: "purse_identifier",
                error,
            }
        })?;
        return Ok(PurseIdentifier::MainPurseUnderAccountHash(account_hash));
    }

    if purse_id.starts_with(UREF_PREFIX) {
        let uref =
            URef::from_formatted_str(purse_id).map_err(|error| CliError::FailedToParseURef {
                context: "purse_identifier",
                error,
            })?;
        return Ok(PurseIdentifier::PurseUref(uref));
    }

    let public_key =
        PublicKey::from_hex(purse_id).map_err(|error| CliError::FailedToParsePublicKey {
            context: "purse_identifier".to_string(),
            error,
        })?;
    Ok(PurseIdentifier::MainPurseUnderPublicKey(public_key))
}

#[cfg(test)]
mod tests {
    use casper_types::{
        account::AccountHash, bytesrepr::ToBytes, AccessRights, CLTyped, CLValue, NamedArg,
        PublicKey, RuntimeArgs, URef, U128, U256, U512,
    };
    use std::convert::TryFrom;

    use super::*;

    mod bad {
        pub const EMPTY: &str = "";
        pub const ARG_UNQUOTED: &str = "name:u32=0"; // value needs single quotes to be valid
        pub const ARG_BAD_TYPE: &str = "name:wat='false'";
        pub const ARG_GIBBERISH: &str = "asdf|1234(..)";
        pub const LARGE_2K_INPUT: &str = r#"
        eJy2irIizK6zT0XOklyBAY1KVUsAbyF6eJUYBmRPHqX2rONbaEieJt4Ci1eZYjBdHdEq46oMBH0LeiQO8RIJb95
        SJGEp83RxakDj7trunJVvMbj2KZFnpJOyEauFa35dlaVG9Ki7hjFy4BLlDyA0Wgwk20RXFkbgKQIQVvR16RPffR
        WO86WqZ3gMuOh447svZRYfhbRF3NVBaWRz7SJ9Zm3w8djisvS0Y3GSnpzKnSEQirApqomfQTHTrU9ww2SMgdGuu
        EllGLsj3ze8WzIbXLlJvXdnJFz7UfsgX4xowG4d6xSiUVWCY4sVItNXlqs8adfZZHH7AjqLjlRRvWwjNCiWsiqx
        ICe9jlkdEVeRAO0BqF6FhjSxPt9X3y6WXAomB0YTIFQGyto4jMBOhWb96ny3DG3WISUSdaKWf8KaRuAQD4ao3ML
        jJZSXkTlovZTYQmYlkYo4s3635YLthuh0hSorRs0ju7ffeY3tu7VRvttgvbBLVjFJjYrwW1YAEOaxDdLnhiTIQn
        H0zRLWnCQ4Czk5BWsRLDdupJbKRWRZcQ7pehSgfc5qtXpJRFVtL2L82hxfBdiXqzXl3KdQ21CnGxTzcgEv0ptrs
        XGJwNgd04YiZzHrZL7iF3xFann6DJVyEZ0eEifTfY8rtxPCMDutjr68iFjnjy40c7SfhvsZLODuEjS4VQkIwfJc
        QP5fH3cQ2K4A4whpzTVc3yqig468Cjbxfobw4Z7YquZnuFw1TXSrM35ZBXpI4WKo9QLxmE2HkgMI1Uac2dWyG0U
        iCAxpHxC4uTIFEq2MUuGd7ZgYs8zoYpODvtAcZ8nUqKssdugQUGfXw9Cs1pcDZgEppYVVw1nYoHXKCjK3oItexs
        uIaZ0m1o91L9Js5lhaDybyDoye9zPFOnEIwKdcH0dO9cZmv6UyvVZS2oVKJm7nHQAJDARjVfC7GYAT2AQhFZxIQ
        DP9jjHCqxMJz6p499G5lk8cYAhnlUm7GCr4AwvjsEU7sEsJcZLDCLG6FaFMdLHJS5v2yPYzpuWebjcNCXbk4yER
        F9NsvlDBrLhoDt1GDgJPlRF8B5h5BSzPHsCjNVa9h2YWx1GVl6Yrrk04FSMSj0nRO8OoxkyU0ugtBQlUv3rQ833
        Vcs7jCGetaazcvaI45dRDGe6LyEPwojlC4IaB8PtljKo2zn0u91lQGJY7rj1qLUtFBRDCKERs7W1j9A2eGJ3ORY
        Db7Q3K7BY9XbANGoYiwtLoytopYCQs5RYHepkoQ19f1E9IcqCFQg9h0rWK494xb88GfSGKBpPHddrQYXFrr715u
        NkAj885V8Mnam5kSzsOmrg504QhPSOaqpkY36xyXUP13yWK4fEf39tJ2PN2DlAsxFAWJUec4CiS47rgrU87oESt
        KZJni3Jhccczlq1CaRKaYYV38joEzPL0UNKr5RiCodTWJmdN07JI5txtQqgc8kvHOrxgOASPQOPSbAUz33vZx3b
        eNsTYUD0Dxa4IkMUNHSy6mpaSOElO7wgUvWJEajnVWZJ5gWehyE4yqo6PkL3VBj51Jg2uozPa8xnbSfymlVVLFl
        EIfMyPwUj1J9ngQw0J3bn33IIOB3bkNfB50f1MkKkhyn1TMZJcnZ7IS16PXBH6DD7Sht1PVKhER2E3QS7z8YQ6B
        q27ktZZ33IcCnayahxHnyf2Wzab9ic5eSJLzsVi0VWP7DePt2GnCbz5D2tcAxgVVFmdIsEakytjmeEGyMu9k2R7
        Q8d1wPtqKgayVtgdIaMbvsnXMkRqITkf3o8Qh495pm1wkKArTGFGODXc1cCKheFUEtJWdK92DHH7OuRENHAb5KS
        PKzSUg2k18wyf9XCy1pQKv31wii3rWrWMCbxOWmhuzw1N9tqO8U97NsThRSoPAjpd05G2roia4m4CaPWTAUmVky
        RfiWoA7bglAh4Aoz2LN2ezFleTNJjjLw3n9bYPg5BdRL8n8wimhXDo9SW46A5YS62C08ZOVtvfn82YRaYkuKKz7
        3NJ25PnQG6diMm4Lm3wi22yR7lY7oYYJjLNcaLYOI6HOvaJ
        "#;
    }

    mod happy {
        pub const HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        pub const NAME: &str = "name";
        pub const PACKAGE_HASH: &str =
            "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        pub const PACKAGE_NAME: &str = "package_name";
        pub const PATH: &str = "./session.wasm";
        pub const ENTRY_POINT: &str = "entrypoint";
        pub const VERSION: &str = "1.0.0";
        pub const TRANSFER: bool = true;
    }

    fn invalid_simple_args_test(cli_string: &str) {
        assert!(
            arg_simple::payment::parse(&[cli_string]).is_err(),
            "{} should be an error",
            cli_string
        );
        assert!(
            arg_simple::session::parse(&[cli_string]).is_err(),
            "{} should be an error",
            cli_string
        );
    }

    fn valid_simple_args_test<T: CLTyped + ToBytes>(cli_string: &str, expected: T) {
        let expected = Some(RuntimeArgs::from(vec![NamedArg::new(
            "x".to_string(),
            CLValue::from_t(expected).unwrap(),
        )]));

        assert_eq!(
            arg_simple::payment::parse(&[cli_string]).expect("should parse"),
            expected
        );
        assert_eq!(
            arg_simple::session::parse(&[cli_string]).expect("should parse"),
            expected
        );
    }

    impl<'a> TryFrom<SessionStrParams<'a>> for ExecutableDeployItem {
        type Error = CliError;

        fn try_from(params: SessionStrParams<'a>) -> Result<ExecutableDeployItem, Self::Error> {
            session_executable_deploy_item(params)
        }
    }

    impl<'a> TryFrom<PaymentStrParams<'a>> for ExecutableDeployItem {
        type Error = CliError;

        fn try_from(params: PaymentStrParams<'a>) -> Result<ExecutableDeployItem, Self::Error> {
            payment_executable_deploy_item(params)
        }
    }

    #[test]
    fn should_parse_bool_via_args_simple() {
        valid_simple_args_test("x:bool='f'", false);
        valid_simple_args_test("x:bool='false'", false);
        valid_simple_args_test("x:bool='t'", true);
        valid_simple_args_test("x:bool='true'", true);
        valid_simple_args_test("x:opt_bool='f'", Some(false));
        valid_simple_args_test("x:opt_bool='t'", Some(true));
        valid_simple_args_test::<Option<bool>>("x:opt_bool=null", None);
    }

    #[test]
    fn should_parse_i32_via_args_simple() {
        valid_simple_args_test("x:i32='2147483647'", i32::max_value());
        valid_simple_args_test("x:i32='0'", 0_i32);
        valid_simple_args_test("x:i32='-2147483648'", i32::min_value());
        valid_simple_args_test("x:opt_i32='-1'", Some(-1_i32));
        valid_simple_args_test::<Option<i32>>("x:opt_i32=null", None);
    }

    #[test]
    fn should_parse_i64_via_args_simple() {
        valid_simple_args_test("x:i64='9223372036854775807'", i64::max_value());
        valid_simple_args_test("x:i64='0'", 0_i64);
        valid_simple_args_test("x:i64='-9223372036854775808'", i64::min_value());
        valid_simple_args_test("x:opt_i64='-1'", Some(-1_i64));
        valid_simple_args_test::<Option<i64>>("x:opt_i64=null", None);
    }

    #[test]
    fn should_parse_u8_via_args_simple() {
        valid_simple_args_test("x:u8='0'", 0_u8);
        valid_simple_args_test("x:u8='255'", u8::max_value());
        valid_simple_args_test("x:opt_u8='1'", Some(1_u8));
        valid_simple_args_test::<Option<u8>>("x:opt_u8=null", None);
    }

    #[test]
    fn should_parse_u32_via_args_simple() {
        valid_simple_args_test("x:u32='0'", 0_u32);
        valid_simple_args_test("x:u32='4294967295'", u32::max_value());
        valid_simple_args_test("x:opt_u32='1'", Some(1_u32));
        valid_simple_args_test::<Option<u32>>("x:opt_u32=null", None);
    }

    #[test]
    fn should_parse_u64_via_args_simple() {
        valid_simple_args_test("x:u64='0'", 0_u64);
        valid_simple_args_test("x:u64='18446744073709551615'", u64::max_value());
        valid_simple_args_test("x:opt_u64='1'", Some(1_u64));
        valid_simple_args_test::<Option<u64>>("x:opt_u64=null", None);
    }

    #[test]
    fn should_parse_u128_via_args_simple() {
        valid_simple_args_test("x:u128='0'", U128::zero());
        valid_simple_args_test(
            "x:u128='340282366920938463463374607431768211455'",
            U128::max_value(),
        );
        valid_simple_args_test("x:opt_u128='1'", Some(U128::from(1)));
        valid_simple_args_test::<Option<U128>>("x:opt_u128=null", None);
    }

    #[test]
    fn should_parse_u256_via_args_simple() {
        valid_simple_args_test("x:u256='0'", U256::zero());
        valid_simple_args_test(
            "x:u256='115792089237316195423570985008687907853269984665640564039457584007913129639935'",
            U256::max_value(),
        );
        valid_simple_args_test("x:opt_u256='1'", Some(U256::from(1)));
        valid_simple_args_test::<Option<U256>>("x:opt_u256=null", None);
    }

    #[test]
    fn should_parse_u512_via_args_simple() {
        valid_simple_args_test("x:u512='0'", U512::zero());
        valid_simple_args_test(
            "x:u512='134078079299425970995740249982058461274793658205923933777235614437217640300735\
            46976801874298166903427690031858186486050853753882811946569946433649006084095'",
            U512::max_value(),
        );
        valid_simple_args_test("x:opt_u512='1'", Some(U512::from(1)));
        valid_simple_args_test::<Option<U512>>("x:opt_u512=null", None);
    }

    #[test]
    fn should_parse_unit_via_args_simple() {
        valid_simple_args_test("x:unit=''", ());
        valid_simple_args_test("x:opt_unit=''", Some(()));
        valid_simple_args_test::<Option<()>>("x:opt_unit=null", None);
    }

    #[test]
    fn should_parse_string_via_args_simple() {
        let value = String::from("test string");
        valid_simple_args_test(&format!("x:string='{}'", value), value.clone());
        valid_simple_args_test(&format!("x:opt_string='{}'", value), Some(value));
        valid_simple_args_test::<Option<String>>("x:opt_string=null", None);
    }

    #[test]
    fn should_parse_key_via_args_simple() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();

        let key_account = Key::Account(AccountHash::new(array));
        let key_hash = Key::Hash(array);
        let key_uref = Key::URef(URef::new(array, AccessRights::NONE));

        for key in &[key_account, key_hash, key_uref] {
            valid_simple_args_test(&format!("x:key='{}'", key.to_formatted_string()), *key);
            valid_simple_args_test(
                &format!("x:opt_key='{}'", key.to_formatted_string()),
                Some(*key),
            );
            valid_simple_args_test::<Option<Key>>("x:opt_key=null", None);
        }
    }

    #[test]
    fn should_parse_account_hash_via_args_simple() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();
        let value = AccountHash::new(array);
        valid_simple_args_test(
            &format!("x:account_hash='{}'", value.to_formatted_string()),
            value,
        );
        valid_simple_args_test(
            &format!("x:opt_account_hash='{}'", value.to_formatted_string()),
            Some(value),
        );
        valid_simple_args_test::<Option<AccountHash>>("x:opt_account_hash=null", None);
    }

    #[test]
    fn should_parse_uref_via_args_simple() {
        let bytes = (1..33).collect::<Vec<_>>();
        let array = <[u8; 32]>::try_from(bytes.as_ref()).unwrap();
        let value = URef::new(array, AccessRights::READ_ADD_WRITE);
        valid_simple_args_test(&format!("x:uref='{}'", value.to_formatted_string()), value);
        valid_simple_args_test(
            &format!("x:opt_uref='{}'", value.to_formatted_string()),
            Some(value),
        );
        valid_simple_args_test::<Option<URef>>("x:opt_uref=null", None);
    }

    #[test]
    fn should_parse_public_key_via_args_simple() {
        let hex_value = "0119bf44096984cdfe8541bac167dc3b96c85086aa30b6b6cb0c5c38ad703166e1";
        let value = PublicKey::from_hex(hex_value).unwrap();
        valid_simple_args_test(&format!("x:public_key='{}'", hex_value), value.clone());
        valid_simple_args_test(&format!("x:opt_public_key='{}'", hex_value), Some(value));
        valid_simple_args_test::<Option<PublicKey>>("x:opt_public_key=null", None);
    }

    #[test]
    fn should_fail_to_parse_bad_args() {
        invalid_simple_args_test(bad::ARG_BAD_TYPE);
        invalid_simple_args_test(bad::ARG_GIBBERISH);
        invalid_simple_args_test(bad::ARG_UNQUOTED);
        invalid_simple_args_test(bad::EMPTY);
        invalid_simple_args_test(bad::LARGE_2K_INPUT);
    }

    #[test]
    fn should_fail_to_parse_conflicting_arg_types() {
        assert!(matches!(
            session_executable_deploy_item(SessionStrParams {
                session_hash: "",
                session_name: "name",
                session_package_hash: "",
                session_package_name: "",
                session_path: "",
                session_args_simple: vec!["something:u32='0'"],
                session_args_complex: "path_to/file",
                session_version: "",
                session_entry_point: "entrypoint",
                is_session_transfer: false
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_session_info",
                ..
            })
        ));

        assert!(matches!(
            payment_executable_deploy_item(PaymentStrParams {
                payment_amount: "",
                payment_hash: "name",
                payment_name: "",
                payment_package_hash: "",
                payment_package_name: "",
                payment_path: "",
                payment_args_simple: vec!["something:u32='0'"],
                payment_args_complex: "path_to/file",
                payment_version: "",
                payment_entry_point: "entrypoint",
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_payment_info",
                ..
            })
        ));
    }

    #[test]
    fn should_fail_to_parse_conflicting_session_parameters() {
        assert!(matches!(
            session_executable_deploy_item(SessionStrParams {
                session_hash: happy::HASH,
                session_name: happy::NAME,
                session_package_hash: happy::PACKAGE_HASH,
                session_package_name: happy::PACKAGE_NAME,
                session_path: happy::PATH,
                session_args_simple: vec![],
                session_args_complex: "",
                session_version: "",
                session_entry_point: "",
                is_session_transfer: false
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_session_info",
                ..
            })
        ));
    }

    #[test]
    fn should_fail_to_parse_conflicting_payment_parameters() {
        assert!(matches!(
            payment_executable_deploy_item(PaymentStrParams {
                payment_amount: "12345",
                payment_hash: happy::HASH,
                payment_name: happy::NAME,
                payment_package_hash: happy::PACKAGE_HASH,
                payment_package_name: happy::PACKAGE_NAME,
                payment_path: happy::PATH,
                payment_args_simple: vec![],
                payment_args_complex: "",
                payment_version: "",
                payment_entry_point: "",
            }),
            Err(CliError::ConflictingArguments {
                context: "parse_payment_info",
                ..
            })
        ));
    }

    #[test]
    fn should_fail_to_parse_bad_session_args_complex() {
        let missing_file = "missing/file";
        assert!(matches!(
            session_executable_deploy_item(SessionStrParams {
                session_hash: happy::HASH,
                session_name: "",
                session_package_hash: "",
                session_package_name: "",
                session_path: "",
                session_args_simple: vec![],
                session_args_complex: missing_file,
                session_version: "",
                session_entry_point: "entrypoint",
                is_session_transfer: false,
            }),
            Err(CliError::Core(crate::Error::IoError {
                context,
                ..
            })) if context == format!("error reading session file at '{}'", missing_file)
        ));
    }

    #[test]
    fn should_fail_to_parse_bad_payment_args_complex() {
        let missing_file = "missing/file";
        assert!(matches!(
            payment_executable_deploy_item(PaymentStrParams {
                payment_amount: "",
                payment_hash: happy::HASH,
                payment_name: "",
                payment_package_hash: "",
                payment_package_name: "",
                payment_path: "",
                payment_args_simple: vec![],
                payment_args_complex: missing_file,
                payment_version: "",
                payment_entry_point: "entrypoint",
            }),
            Err(CliError::Core(crate::Error::IoError {
                context,
                ..
            })) if context == format!("error reading payment file at '{}'", missing_file)
        ));
    }

    mod missing_args {
        use super::*;

        #[test]
        fn session_name_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_name: happy::NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_hash_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_hash: happy::HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_package_hash_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_package_hash: happy::PACKAGE_HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn session_package_name_should_fail_to_parse_missing_entry_point() {
            let result = session_executable_deploy_item(SessionStrParams {
                session_package_name: happy::PACKAGE_NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_session_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_name_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_name: happy::NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_hash_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_hash: happy::HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_package_hash_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_package_hash: happy::PACKAGE_HASH,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }

        #[test]
        fn payment_package_name_should_fail_to_parse_missing_entry_point() {
            let result = payment_executable_deploy_item(PaymentStrParams {
                payment_package_name: happy::PACKAGE_NAME,
                ..Default::default()
            });

            assert!(matches!(
                result,
                Err(CliError::InvalidArgument {
                    context: "parse_payment_info",
                    ..
                })
            ));
        }
    }

    mod conflicting_args {
        use super::*;

        /// impl_test_matrix - implements many tests for SessionStrParams or PaymentStrParams which
        /// ensures that an error is returned when the permutation they define is executed.
        ///
        /// For instance, it is neccesary to check that when `session_path` is set, other arguments
        /// are not.
        ///
        /// For example, a sample invocation with one test:
        /// ```
        /// impl_test_matrix![
        ///     type: SessionStrParams,
        ///     context: "parse_session_info",
        ///     session_str_params[
        ///         test[
        ///             session_path => happy::PATH,
        ///             conflict: session_package_hash => happy::PACKAGE_HASH,
        ///             requires[],
        ///             path_conflicts_with_package_hash
        ///         ]
        ///     ]
        /// ];
        /// ```
        /// This generates the following test module (with the fn name passed), with one test per
        /// line in `session_str_params[]`:
        /// ```
        /// #[cfg(test)]
        /// mod session_str_params {
        ///     use super::*;
        ///
        ///     #[test]
        ///     fn path_conflicts_with_package_hash() {
        ///         let info: StdResult<ExecutableDeployItem, _> = SessionStrParams {
        ///                 session_path: happy::PATH,
        ///                 session_package_hash: happy::PACKAGE_HASH,
        ///                 ..Default::default()
        ///             }
        ///             .try_into();
        ///         let mut conflicting = vec![
        ///             format!("{}={}", "session_path", happy::PATH),
        ///             format!("{}={}", "session_package_hash", happy::PACKAGE_HASH),
        ///         ];
        ///         conflicting.sort();
        ///         assert!(matches!(
        ///             info,
        ///             Err(CliError::ConflictingArguments {
        ///                 context: "parse_session_info",
        ///                 args: conflicting
        ///             }
        ///             ))
        ///         );
        ///     }
        /// }
        /// ```
        macro_rules! impl_test_matrix {
            (
                /// Struct for which to define the following tests. In our case, SessionStrParams or PaymentStrParams.
                type: $t:ident,
                /// Expected `context` field to be returned in the `CliError::ConflictingArguments{ context, .. }` field.
                context: $context:expr,

                /// $module will be our module name.
                $module:ident [$(
                    // many tests can be defined
                    test[
                        /// The argument's ident to be tested, followed by it's value.
                        $arg:tt => $arg_value:expr,
                        /// The conflicting argument's ident to be tested, followed by it's value.
                        conflict: $con:tt => $con_value:expr,
                        /// A list of any additional fields required by the argument, and their values.
                        requires[$($req:tt => $req_value:expr),*],
                        /// fn name for the defined test.
                        $test_fn_name:ident
                    ]
                )+]
            ) => {
                #[cfg(test)]
                mod $module {
                    use super::*;

                    $(
                        #[test]
                        fn $test_fn_name() {
                            let info: Result<ExecutableDeployItem, _> = $t {
                                $arg: $arg_value,
                                $con: $con_value,
                                $($req: $req_value,),*
                                ..Default::default()
                            }
                            .try_into();
                            let mut conflicting = vec![
                                format!("{}={}", stringify!($arg), $arg_value),
                                format!("{}={}", stringify!($con), $con_value),
                            ];
                            conflicting.sort();
                            assert!(matches!(
                                info,
                                Err(CliError::ConflictingArguments {
                                    context: $context,
                                    ..
                                }
                                ))
                            );
                        }
                    )+
                }
            };
        }

        // NOTE: there's no need to test a conflicting argument in both directions, since they
        // amount to passing two fields to a structs constructor.
        // Where a reverse test like this is omitted, a comment should be left.
        impl_test_matrix![
            type: SessionStrParams,
            context: "parse_session_info",
            session_str_params[

                // path
                test[session_path => happy::PATH, conflict: session_package_hash => happy::PACKAGE_HASH, requires[], path_conflicts_with_package_hash]
                test[session_path => happy::PATH, conflict: session_package_name => happy::PACKAGE_NAME, requires[], path_conflicts_with_package_name]
                test[session_path => happy::PATH, conflict: session_hash =>         happy::HASH,         requires[], path_conflicts_with_hash]
                test[session_path => happy::PATH, conflict: session_name =>         happy::HASH,         requires[], path_conflicts_with_name]
                test[session_path => happy::PATH, conflict: session_version =>      happy::VERSION,      requires[], path_conflicts_with_version]
                test[session_path => happy::PATH, conflict: session_entry_point =>  happy::ENTRY_POINT,  requires[], path_conflicts_with_entry_point]
                test[session_path => happy::PATH, conflict: is_session_transfer =>  happy::TRANSFER,     requires[], path_conflicts_with_transfer]

                // name
                test[session_name => happy::NAME, conflict: session_package_hash => happy::PACKAGE_HASH, requires[session_entry_point => happy::ENTRY_POINT], name_conflicts_with_package_hash]
                test[session_name => happy::NAME, conflict: session_package_name => happy::PACKAGE_NAME, requires[session_entry_point => happy::ENTRY_POINT], name_conflicts_with_package_name]
                test[session_name => happy::NAME, conflict: session_hash =>         happy::HASH,         requires[session_entry_point => happy::ENTRY_POINT], name_conflicts_with_hash]
                test[session_name => happy::NAME, conflict: session_version =>      happy::VERSION,      requires[session_entry_point => happy::ENTRY_POINT], name_conflicts_with_version]
                test[session_name => happy::NAME, conflict: is_session_transfer =>  happy::TRANSFER,     requires[session_entry_point => happy::ENTRY_POINT], name_conflicts_with_transfer]

                // hash
                test[session_hash => happy::HASH, conflict: session_package_hash => happy::PACKAGE_HASH, requires[session_entry_point => happy::ENTRY_POINT], hash_conflicts_with_package_hash]
                test[session_hash => happy::HASH, conflict: session_package_name => happy::PACKAGE_NAME, requires[session_entry_point => happy::ENTRY_POINT], hash_conflicts_with_package_name]
                test[session_hash => happy::HASH, conflict: session_version =>      happy::VERSION,      requires[session_entry_point => happy::ENTRY_POINT], hash_conflicts_with_version]
                test[session_hash => happy::HASH, conflict: is_session_transfer =>  happy::TRANSFER,     requires[session_entry_point => happy::ENTRY_POINT], hash_conflicts_with_transfer]
                // name <-> hash is already checked
                // name <-> path is already checked

                // package_name
                // package_name + session_version is optional and allowed
                test[session_package_name => happy::PACKAGE_NAME, conflict: session_package_hash => happy::PACKAGE_HASH, requires[session_entry_point => happy::ENTRY_POINT], package_name_conflicts_with_package_hash]
                test[session_package_name => happy::VERSION, conflict: is_session_transfer => happy::TRANSFER, requires[session_entry_point => happy::ENTRY_POINT], package_name_conflicts_with_transfer]
                // package_name <-> hash is already checked
                // package_name <-> name is already checked
                // package_name <-> path is already checked

                // package_hash
                // package_hash + session_version is optional and allowed
                test[session_package_hash => happy::PACKAGE_HASH, conflict: is_session_transfer => happy::TRANSFER, requires[session_entry_point => happy::ENTRY_POINT], package_hash_conflicts_with_transfer]
                // package_hash <-> package_name is already checked
                // package_hash <-> hash is already checked
                // package_hash <-> name is already checked
                // package_hash <-> path is already checked

            ]
        ];

        impl_test_matrix![
            type: PaymentStrParams,
            context: "parse_payment_info",
            payment_str_params[

                // amount
                test[payment_amount => happy::PATH, conflict: payment_package_hash => happy::PACKAGE_HASH, requires[], amount_conflicts_with_package_hash]
                test[payment_amount => happy::PATH, conflict: payment_package_name => happy::PACKAGE_NAME, requires[], amount_conflicts_with_package_name]
                test[payment_amount => happy::PATH, conflict: payment_hash =>         happy::HASH,         requires[], amount_conflicts_with_hash]
                test[payment_amount => happy::PATH, conflict: payment_name =>         happy::HASH,         requires[], amount_conflicts_with_name]
                test[payment_amount => happy::PATH, conflict: payment_version =>      happy::VERSION,      requires[], amount_conflicts_with_version]
                test[payment_amount => happy::PATH, conflict: payment_entry_point =>  happy::ENTRY_POINT,  requires[], amount_conflicts_with_entry_point]

                // path
                // amount <-> path is already checked
                test[payment_path => happy::PATH, conflict: payment_package_hash => happy::PACKAGE_HASH, requires[], path_conflicts_with_package_hash]
                test[payment_path => happy::PATH, conflict: payment_package_name => happy::PACKAGE_NAME, requires[], path_conflicts_with_package_name]
                test[payment_path => happy::PATH, conflict: payment_hash =>         happy::HASH,         requires[], path_conflicts_with_hash]
                test[payment_path => happy::PATH, conflict: payment_name =>         happy::HASH,         requires[], path_conflicts_with_name]
                test[payment_path => happy::PATH, conflict: payment_version =>      happy::VERSION,      requires[], path_conflicts_with_version]
                test[payment_path => happy::PATH, conflict: payment_entry_point =>  happy::ENTRY_POINT,  requires[], path_conflicts_with_entry_point]

                // name
                // amount <-> path is already checked
                test[payment_name => happy::NAME, conflict: payment_package_hash => happy::PACKAGE_HASH, requires[payment_entry_point => happy::ENTRY_POINT], name_conflicts_with_package_hash]
                test[payment_name => happy::NAME, conflict: payment_package_name => happy::PACKAGE_NAME, requires[payment_entry_point => happy::ENTRY_POINT], name_conflicts_with_package_name]
                test[payment_name => happy::NAME, conflict: payment_hash =>         happy::HASH,         requires[payment_entry_point => happy::ENTRY_POINT], name_conflicts_with_hash]
                test[payment_name => happy::NAME, conflict: payment_version =>      happy::VERSION,      requires[payment_entry_point => happy::ENTRY_POINT], name_conflicts_with_version]

                // hash
                // amount <-> hash is already checked
                test[payment_hash => happy::HASH, conflict: payment_package_hash => happy::PACKAGE_HASH, requires[payment_entry_point => happy::ENTRY_POINT], hash_conflicts_with_package_hash]
                test[payment_hash => happy::HASH, conflict: payment_package_name => happy::PACKAGE_NAME, requires[payment_entry_point => happy::ENTRY_POINT], hash_conflicts_with_package_name]
                test[payment_hash => happy::HASH, conflict: payment_version =>      happy::VERSION,      requires[payment_entry_point => happy::ENTRY_POINT], hash_conflicts_with_version]
                // name <-> hash is already checked
                // name <-> path is already checked

                // package_name
                // amount <-> package_name is already checked
                test[payment_package_name => happy::PACKAGE_NAME, conflict: payment_package_hash => happy::PACKAGE_HASH, requires[payment_entry_point => happy::ENTRY_POINT], package_name_conflicts_with_package_hash]
                // package_name <-> hash is already checked
                // package_name <-> name is already checked
                // package_name <-> path is already checked

                // package_hash
                // package_hash + session_version is optional and allowed
                // amount <-> package_hash is already checked
                // package_hash <-> package_name is already checked
                // package_hash <-> hash is already checked
                // package_hash <-> name is already checked
                // package_hash <-> path is already checked
            ]
        ];
    }

    mod param_tests {
        use super::*;

        const HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        const NAME: &str = "name";
        const PKG_NAME: &str = "pkg_name";
        const PKG_HASH: &str = "09dcee4b212cfd53642ab323fbef07dafafc6f945a80a00147f62910a915c4e6";
        const ENTRYPOINT: &str = "entrypoint";
        const VERSION: &str = "0.1.0";

        fn args_simple() -> Vec<&'static str> {
            vec!["name_01:bool='false'", "name_02:u32='42'"]
        }

        /// Sample data creation methods for PaymentStrParams
        mod session_params {
            use std::collections::BTreeMap;

            use casper_types::CLValue;

            use super::*;

            #[test]
            pub fn with_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_hash(HASH, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_name(NAME, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_package_name(
                        PKG_NAME,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    SessionStrParams::with_package_hash(
                        PKG_HASH,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }
        }

        /// Sample data creation methods for PaymentStrParams
        mod payment_params {
            use std::collections::BTreeMap;

            use casper_types::{CLValue, U512};

            use super::*;

            #[test]
            pub fn with_amount() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_amount("100").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::ModuleBytes { .. }) => {
                        let amount = CLValue::from_t(U512::from(100)).unwrap();
                        assert_eq!(item.args().get("amount"), Some(&amount));
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_hash(HASH, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_name(NAME, ENTRYPOINT, args_simple(), "").try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_name() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_package_name(
                        PKG_NAME,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByName { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }

            #[test]
            pub fn with_package_hash() {
                let params: Result<ExecutableDeployItem, CliError> =
                    PaymentStrParams::with_package_hash(
                        PKG_HASH,
                        VERSION,
                        ENTRYPOINT,
                        args_simple(),
                        "",
                    )
                    .try_into();
                match params {
                    Ok(item @ ExecutableDeployItem::StoredVersionedContractByHash { .. }) => {
                        let actual: BTreeMap<String, CLValue> = item.args().clone().into();
                        let mut expected = BTreeMap::new();
                        expected.insert("name_01".to_owned(), CLValue::from_t(false).unwrap());
                        expected.insert("name_02".to_owned(), CLValue::from_t(42u32).unwrap());
                        assert_eq!(actual, expected);
                    }
                    other => panic!("incorrect type parsed {:?}", other),
                }
            }
        }
    }
}
