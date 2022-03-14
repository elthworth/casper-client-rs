use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_types::{ContractPackageHash, ContractWasmHash, EntryPoint, ProtocolVersion};

use super::NamedKey;

/// Details of a smart contract.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    contract_package_hash: ContractPackageHash,
    contract_wasm_hash: ContractWasmHash,
    named_keys: Vec<NamedKey>,
    entry_points: Vec<EntryPoint>,
    protocol_version: ProtocolVersion,
}

impl Contract {
    /// Returns the contract package hash of the contract.
    pub fn contract_package_hash(&self) -> &ContractPackageHash {
        &self.contract_package_hash
    }

    /// Returns the contract Wasm hash of the contract.
    pub fn contract_wasm_hash(&self) -> &ContractWasmHash {
        &self.contract_wasm_hash
    }

    /// Returns the named keys of the contract.
    pub fn named_keys(&self) -> impl Iterator<Item = &NamedKey> {
        self.named_keys.iter()
    }

    /// Returns the entry-points of the contract.
    pub fn entry_points(&self) -> impl Iterator<Item = &EntryPoint> {
        self.entry_points.iter()
    }

    /// Returns the protocol version of the contract.
    pub fn protocol_version(&self) -> &ProtocolVersion {
        &self.protocol_version
    }
}

impl Display for Contract {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "contract {{ package-hash {}, wasm-hash {}, named keys {{{}}}, entry-points \
            {{{}}}, protocol-version: {} }}",
            self.contract_package_hash,
            self.contract_wasm_hash,
            self.named_keys.iter().format(", "),
            self.entry_points
                .iter()
                .format_with(", ", |entry_point, fmt_fn| {
                    fmt_fn(&format_args!(
                        "{{ {}, parameters {{{}}}, ret {:?}, access {:?}, type {:?} }}",
                        entry_point.name(),
                        entry_point
                            .args()
                            .iter()
                            .format_with(", ", |param, fmt_fn| {
                                fmt_fn(&format_args!(
                                    "{{ {}, {:?} }}",
                                    param.name(),
                                    param.cl_type()
                                ))
                            }),
                        entry_point.ret(),
                        entry_point.access(),
                        entry_point.entry_point_type()
                    ))
                }),
            self.protocol_version
        )
    }
}
