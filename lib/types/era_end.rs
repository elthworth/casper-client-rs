use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

use serde::{Deserialize, Serialize};

use casper_types::{
    bytesrepr::{self, ToBytes},
    crypto::PublicKey,
    U512,
};

/// A reward to be given to the specified validator.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Reward {
    validator: PublicKey,
    amount: u64,
}

impl Reward {
    /// Returns the validator's public key.
    pub fn validator(&self) -> &PublicKey {
        &self.validator
    }

    /// Returns the number of rewarded motes.
    pub fn amount(&self) -> u64 {
        self.amount
    }
}

/// Equivocation and reward information included in switch blocks.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EraReport {
    pub(super) equivocators: Vec<PublicKey>,
    pub(super) rewards: Vec<Reward>,
    pub(super) inactive_validators: Vec<PublicKey>,
}

impl EraReport {
    /// Returns the collection of validators which have equivocated in this era.
    pub fn equivocators(&self) -> impl Iterator<Item = &PublicKey> {
        self.equivocators.iter()
    }

    /// Returns the collection of rewards due.
    pub fn rewards(&self) -> impl Iterator<Item = &Reward> {
        self.rewards.iter()
    }

    /// Returns the collection of validators which were marked inactive in this era.
    pub fn inactive_validators(&self) -> impl Iterator<Item = &PublicKey> {
        self.inactive_validators.iter()
    }
}

impl ToBytes for EraReport {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        let rewards: BTreeMap<PublicKey, u64> = self
            .rewards
            .iter()
            .map(|reward| (reward.validator.clone(), reward.amount))
            .collect();

        self.equivocators.write_bytes(buffer)?;
        rewards.write_bytes(buffer)?;
        self.inactive_validators.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        let rewards: BTreeMap<PublicKey, u64> = self
            .rewards
            .iter()
            .map(|reward| (reward.validator.clone(), reward.amount))
            .collect();
        self.equivocators.serialized_length()
            + rewards.serialized_length()
            + self.inactive_validators.serialized_length()
    }
}

/// The amount of weight a given validator has.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValidatorWeight {
    validator: PublicKey,
    weight: U512,
}

impl ValidatorWeight {
    /// Returns the validator's public key.
    pub fn validator(&self) -> &PublicKey {
        &self.validator
    }

    /// Returns the validator's weight.
    pub fn weight(&self) -> U512 {
        self.weight
    }
}

/// Information included in switch blocks about the era the block concludes and the subsequent era.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EraEnd {
    /// The report on the era just concluded.
    pub(super) era_report: EraReport,
    /// The validator weights for the subsequent era.
    pub(super) next_era_validator_weights: Vec<ValidatorWeight>,
}

impl EraEnd {
    /// Returns the report on the era just concluded.
    pub fn era_report(&self) -> &EraReport {
        &self.era_report
    }

    /// Returns the validator weights for the subsequent era.
    pub fn next_era_validator_weights(&self) -> impl Iterator<Item = &ValidatorWeight> {
        self.next_era_validator_weights.iter()
    }
}

impl Display for EraEnd {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{:?}", self.era_report)
    }
}

impl ToBytes for EraEnd {
    fn write_bytes(&self, buffer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        let next_era_validator_weights: BTreeMap<PublicKey, U512> = self
            .next_era_validator_weights
            .iter()
            .map(|validator_weight| (validator_weight.validator.clone(), validator_weight.weight))
            .collect();

        self.era_report.write_bytes(buffer)?;
        next_era_validator_weights.write_bytes(buffer)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = vec![];
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        let next_era_validator_weights: BTreeMap<PublicKey, U512> = self
            .next_era_validator_weights
            .iter()
            .map(|validator_weight| (validator_weight.validator.clone(), validator_weight.weight))
            .collect();
        self.era_report.serialized_length() + next_era_validator_weights.serialized_length()
    }
}
