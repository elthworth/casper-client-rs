use hex_buffer_serde::{Hex, HexForm};
use serde::{Deserialize, Serialize};

use casper_types::{
    bytesrepr::{self, Bytes, ToBytes},
    runtime_args, ContractHash, ContractPackageHash, ContractVersion, RuntimeArgs, URef, U512,
};

#[cfg(doc)]
use crate::Deploy;
use crate::TransferTarget;

const TAG_LENGTH: usize = 1;
const MODULE_BYTES_TAG: u8 = 0;
const STORED_CONTRACT_BY_HASH_TAG: u8 = 1;
const STORED_CONTRACT_BY_NAME_TAG: u8 = 2;
const STORED_VERSIONED_CONTRACT_BY_HASH_TAG: u8 = 3;
const STORED_VERSIONED_CONTRACT_BY_NAME_TAG: u8 = 4;
const TRANSFER_TAG: u8 = 5;
const STANDARD_PAYMENT_ARG_AMOUNT: &str = "amount";
const TRANSFER_ARG_AMOUNT: &str = "amount";
const TRANSFER_ARG_SOURCE: &str = "source";
const TRANSFER_ARG_TARGET: &str = "target";
const TRANSFER_ARG_ID: &str = "id";

/// The payment or session code of a [`Deploy`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum ExecutableDeployItem {
    /// Raw bytes of compiled Wasm code, which must include a `call` entry point, and the arguments
    /// to call at runtime.
    ModuleBytes {
        /// The compiled Wasm bytes.
        module_bytes: Bytes,
        /// The arguments to be passed to the entry point at runtime.
        args: RuntimeArgs,
    },
    /// A contract stored in global state, referenced by its "hash", along with the entry point and
    /// arguments to call at runtime.
    StoredContractByHash {
        /// The contract's identifier.
        #[serde(with = "HexForm")]
        hash: ContractHash,
        /// The contract's entry point to be called at runtime.
        entry_point: String,
        /// The arguments to be passed to the entry point at runtime.
        args: RuntimeArgs,
    },
    /// A contract stored in global state, referenced by a named key existing in the `Deploy`'s
    /// account context, along with the entry point and arguments to call at runtime.
    StoredContractByName {
        /// The named of the named key under which the contract is referenced.
        name: String,
        /// The contract's entry point to be called at runtime.
        entry_point: String,
        /// The arguments to be passed to the entry point at runtime.
        args: RuntimeArgs,
    },
    /// A versioned contract stored in global state, referenced by its "hash", along with the entry
    /// point and arguments to call at runtime.
    StoredVersionedContractByHash {
        /// The contract package's identifier.
        #[serde(with = "HexForm")]
        hash: ContractPackageHash,
        /// The version of the contract to call.  If `None`, the highest enabled version is used.
        version: Option<ContractVersion>,
        /// The contract's entry point to be called at runtime.
        entry_point: String,
        /// The arguments to be passed to the entry point at runtime.
        args: RuntimeArgs,
    },
    /// A versioned contract stored in global state, referenced by a named key existing in the
    /// `Deploy`'s account context, along with the entry point and arguments to call at runtime.
    StoredVersionedContractByName {
        /// The named of the named key under which the contract package is referenced.
        name: String,
        /// The version of the contract to call.  If `None`, the highest enabled version is used.
        version: Option<ContractVersion>,
        /// The contract's entry point to be called at runtime.
        entry_point: String,
        /// The arguments to be passed to the entry point at runtime.
        args: RuntimeArgs,
    },
    /// A native transfer which does not contain or reference any Wasm code.
    Transfer {
        /// The arguments to be passed to the native transfer entry point at runtime.
        args: RuntimeArgs,
    },
}

impl ExecutableDeployItem {
    /// Returns a new `ExecutableDeployItem::ModuleBytes`.
    pub fn new_module_bytes(module_bytes: Bytes, args: RuntimeArgs) -> Self {
        ExecutableDeployItem::ModuleBytes { module_bytes, args }
    }

    /// Returns a new `ExecutableDeployItem::ModuleBytes` suitable for use as standard payment code
    /// of a `Deploy`.
    pub fn new_standard_payment<A: Into<U512>>(amount: A) -> Self {
        ExecutableDeployItem::ModuleBytes {
            module_bytes: Bytes::new(),
            args: runtime_args! {
                STANDARD_PAYMENT_ARG_AMOUNT => amount.into(),
            },
        }
    }

    /// Returns a new `ExecutableDeployItem::StoredContractByHash`.
    pub fn new_stored_contract_by_hash(
        hash: ContractHash,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Self {
        ExecutableDeployItem::StoredContractByHash {
            hash,
            entry_point,
            args,
        }
    }

    /// Returns a new `ExecutableDeployItem::StoredContractByName`.
    pub fn new_stored_contract_by_name(
        name: String,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Self {
        ExecutableDeployItem::StoredContractByName {
            name,
            entry_point,
            args,
        }
    }

    /// Returns a new `ExecutableDeployItem::StoredVersionedContractByHash`.
    pub fn new_stored_versioned_contract_by_hash(
        hash: ContractPackageHash,
        version: Option<ContractVersion>,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Self {
        ExecutableDeployItem::StoredVersionedContractByHash {
            hash,
            version,
            entry_point,
            args,
        }
    }

    /// Returns a new `ExecutableDeployItem::StoredVersionedContractByName`.
    pub fn new_stored_versioned_contract_by_name(
        name: String,
        version: Option<ContractVersion>,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Self {
        ExecutableDeployItem::StoredVersionedContractByName {
            name,
            version,
            entry_point,
            args,
        }
    }

    /// Returns a new `ExecutableDeployItem` suitable for use as session code for a transfer.
    ///
    /// If `maybe_source` is None, the account's main purse is used as the source.
    pub fn new_transfer<A: Into<U512>>(
        amount: A,
        maybe_source: Option<URef>,
        target: TransferTarget,
        maybe_transfer_id: Option<u64>,
    ) -> Self {
        let mut args = RuntimeArgs::new();
        args.insert(TRANSFER_ARG_AMOUNT, amount.into())
            .expect("should serialize amount arg");

        if let Some(source) = maybe_source {
            args.insert(TRANSFER_ARG_SOURCE, source)
                .expect("should serialize source arg");
        }

        match target {
            TransferTarget::PublicKey(public_key) => args
                .insert(TRANSFER_ARG_TARGET, public_key)
                .expect("should serialize public key target arg"),
            TransferTarget::AccountHash(account_hash) => args
                .insert(TRANSFER_ARG_TARGET, account_hash)
                .expect("should serialize account hash target arg"),
            TransferTarget::URef(uref) => args
                .insert(TRANSFER_ARG_TARGET, uref)
                .expect("should serialize uref target arg"),
        }

        args.insert(TRANSFER_ARG_ID, maybe_transfer_id)
            .expect("should serialize transfer id arg");

        ExecutableDeployItem::Transfer { args }
    }

    /// Returns the runtime arguments.
    #[cfg(test)]
    pub(crate) fn args(&self) -> &RuntimeArgs {
        match self {
            ExecutableDeployItem::ModuleBytes { args, .. }
            | ExecutableDeployItem::StoredContractByHash { args, .. }
            | ExecutableDeployItem::StoredContractByName { args, .. }
            | ExecutableDeployItem::StoredVersionedContractByHash { args, .. }
            | ExecutableDeployItem::StoredVersionedContractByName { args, .. }
            | ExecutableDeployItem::Transfer { args } => args,
        }
    }
}

impl ToBytes for ExecutableDeployItem {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        match self {
            ExecutableDeployItem::ModuleBytes { module_bytes, args } => {
                buffer.push(MODULE_BYTES_TAG);
                module_bytes.write_bytes(buffer)?;
                args.write_bytes(buffer)
            }
            ExecutableDeployItem::StoredContractByHash {
                hash,
                entry_point,
                args,
            } => {
                buffer.push(STORED_CONTRACT_BY_HASH_TAG);
                hash.write_bytes(buffer)?;
                entry_point.write_bytes(buffer)?;
                args.write_bytes(buffer)
            }
            ExecutableDeployItem::StoredContractByName {
                name,
                entry_point,
                args,
            } => {
                buffer.push(STORED_CONTRACT_BY_NAME_TAG);
                name.write_bytes(buffer)?;
                entry_point.write_bytes(buffer)?;
                args.write_bytes(buffer)
            }
            ExecutableDeployItem::StoredVersionedContractByHash {
                hash,
                version,
                entry_point,
                args,
            } => {
                buffer.push(STORED_VERSIONED_CONTRACT_BY_HASH_TAG);
                hash.write_bytes(buffer)?;
                version.write_bytes(buffer)?;
                entry_point.write_bytes(buffer)?;
                args.write_bytes(buffer)
            }
            ExecutableDeployItem::StoredVersionedContractByName {
                name,
                version,
                entry_point,
                args,
            } => {
                buffer.push(STORED_VERSIONED_CONTRACT_BY_NAME_TAG);
                name.write_bytes(buffer)?;
                version.write_bytes(buffer)?;
                entry_point.write_bytes(buffer)?;
                args.write_bytes(buffer)
            }
            ExecutableDeployItem::Transfer { args } => {
                buffer.push(TRANSFER_TAG);
                args.write_bytes(buffer)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        TAG_LENGTH
            + match self {
                ExecutableDeployItem::ModuleBytes { module_bytes, args } => {
                    module_bytes.serialized_length() + args.serialized_length()
                }
                ExecutableDeployItem::StoredContractByHash {
                    hash,
                    entry_point,
                    args,
                } => {
                    hash.serialized_length()
                        + entry_point.serialized_length()
                        + args.serialized_length()
                }
                ExecutableDeployItem::StoredContractByName {
                    name,
                    entry_point,
                    args,
                } => {
                    name.serialized_length()
                        + entry_point.serialized_length()
                        + args.serialized_length()
                }
                ExecutableDeployItem::StoredVersionedContractByHash {
                    hash,
                    version,
                    entry_point,
                    args,
                } => {
                    hash.serialized_length()
                        + version.serialized_length()
                        + entry_point.serialized_length()
                        + args.serialized_length()
                }
                ExecutableDeployItem::StoredVersionedContractByName {
                    name,
                    version,
                    entry_point,
                    args,
                } => {
                    name.serialized_length()
                        + version.serialized_length()
                        + entry_point.serialized_length()
                        + args.serialized_length()
                }
                ExecutableDeployItem::Transfer { args } => args.serialized_length(),
            }
    }
}
