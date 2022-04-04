use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_types::{ContractHash, URef};

/// Contract version.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractVersion {
    protocol_version_major: u32,
    contract_version: u32,
    contract_hash: ContractHash,
}

impl ContractVersion {
    /// Returns the major protocol version of the contract.
    pub fn protocol_version_major(&self) -> u32 {
        self.protocol_version_major
    }

    /// Returns the version of the contract.
    pub fn contract_version(&self) -> u32 {
        self.contract_version
    }

    /// Returns the hash used to identify the contract.
    pub fn contract_hash(&self) -> &ContractHash {
        &self.contract_hash
    }
}

impl Display for ContractVersion {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "contract-version {{ protocol-major {}, version {}, hash {} }}",
            self.protocol_version_major, self.contract_version, self.contract_hash
        )
    }
}

/// Disabled contract version.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DisabledVersion {
    protocol_version_major: u32,
    contract_version: u32,
}

impl DisabledVersion {
    /// Returns the major protocol version of the disabled contract.
    pub fn protocol_version_major(&self) -> u32 {
        self.protocol_version_major
    }

    /// Returns the version of the disabled contract.
    pub fn contract_version(&self) -> u32 {
        self.contract_version
    }
}

impl Display for DisabledVersion {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "disabled contract-version {{ protocol-major {}, version {} }}",
            self.protocol_version_major, self.contract_version
        )
    }
}

/// A set of `URef`s associated with a named user group.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Group {
    group: String,
    keys: Vec<URef>,
}

impl Group {
    /// Returns the name of the group.
    pub fn group(&self) -> &str {
        &self.group
    }

    /// Returns the URefs of the group.
    pub fn keys(&self) -> impl Iterator<Item = &URef> {
        self.keys.iter()
    }
}

impl Display for Group {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "group {{ {}: {{{}}} }}",
            self.group,
            self.keys().format(", ")
        )
    }
}

/// Contract definition, metadata, and security container.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ContractPackage {
    access_key: URef,
    versions: Vec<ContractVersion>,
    disabled_versions: Vec<DisabledVersion>,
    groups: Vec<Group>,
}

impl ContractPackage {
    /// Returns the access key of the contract.
    pub fn access_key(&self) -> &URef {
        &self.access_key
    }

    /// Returns all versions of the contract.
    pub fn versions(&self) -> impl Iterator<Item = &ContractVersion> {
        self.versions.iter()
    }

    /// Returns the disabled versions of the contract.
    pub fn disabled_versions(&self) -> impl Iterator<Item = &DisabledVersion> {
        self.disabled_versions.iter()
    }

    /// Returns the user groups of the contract.
    pub fn groups(&self) -> impl Iterator<Item = &Group> {
        self.groups.iter()
    }
}

impl Display for ContractPackage {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "contract-package {{ {}, all versions {{{}}}, disabled versions {{{}}}, groups {{{}}} \
            }}",
            self.access_key,
            self.versions().format(", "),
            self.disabled_versions().format(", "),
            self.groups().format(", "),
        )
    }
}
