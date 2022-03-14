use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[cfg(doc)]
use casper_types::CLTyped;
use casper_types::{
    system::auction::{Bid, EraInfo, UnbondingPurse, WithdrawPurse},
    CLValue, DeployInfo, Transfer,
};

use super::{Account, Contract, ContractPackage};

/// A value stored in global state.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum StoredValue {
    /// A CLTyped value.
    CLValue(CLValue),
    /// An account.
    Account(Account),
    /// A contract's Wasm, serialized via bytesrepr, then hex-encoded.
    ContractWasm(String),
    /// Methods and type signatures supported by a contract.
    Contract(Contract),
    /// A contract definition, metadata, and security container.
    ContractPackage(ContractPackage),
    /// A record of a transfer.
    Transfer(Transfer),
    /// A record of a deploy.
    DeployInfo(DeployInfo),
    /// Auction metadata.
    EraInfo(EraInfo),
    /// A bid.
    Bid(Box<Bid>),
    /// A record of withdrawal information.
    Withdraw(Vec<WithdrawPurse>),
    /// A record of unbonding information.
    Unbonding(Vec<UnbondingPurse>),
}

impl Display for StoredValue {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            // TODO - improve this variant.
            StoredValue::CLValue(cl_value) => {
                write!(formatter, "stored value {{ {:?} }}", cl_value)
            }
            StoredValue::Account(account) => {
                write!(formatter, "stored value {{ {} }}", account)
            }
            StoredValue::ContractWasm(hex_chars) => {
                write!(
                    formatter,
                    "stored value {{ contract-wasm {} hex chars }}",
                    hex_chars.len()
                )
            }
            StoredValue::Contract(contract) => {
                write!(formatter, "stored value {{ {} }}", contract)
            }
            StoredValue::ContractPackage(contract_package) => {
                write!(formatter, "stored value {{ {} }}", contract_package)
            }
            // TODO - improve this variant.
            StoredValue::Transfer(transfer) => {
                write!(formatter, "stored value {{ {:?} }}", transfer)
            }
            // TODO - improve this variant.
            StoredValue::DeployInfo(deploy_info) => {
                write!(formatter, "stored value {{ {:?} }}", deploy_info)
            }
            // TODO - improve this variant.
            StoredValue::EraInfo(era_info) => {
                write!(formatter, "stored value {{ {:?} }}", era_info)
            }
            // TODO - improve this variant.
            StoredValue::Bid(bid) => {
                write!(formatter, "stored value {{ {:?} }}", bid)
            }
            // TODO - improve this variant.
            StoredValue::Withdraw(withdrawal_info) => {
                write!(formatter, "stored value {{ {:?} }}", withdrawal_info)
            }
            // TODO - improve this variant.
            StoredValue::Unbonding(unbonding_info) => {
                write!(formatter, "stored value {{ {:?} }}", unbonding_info)
            }
        }
    }
}
