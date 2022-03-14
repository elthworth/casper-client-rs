use serde::{Deserialize, Serialize};

use casper_types::{EraId, ProtocolVersion, PublicKey};

pub(crate) const GET_VALIDATOR_CHANGES_METHOD: &str = "info_get_validator_changes";

/// A change to a validator's status between two eras.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum ValidatorChange {
    /// The validator got newly added to the validator set.
    Added,
    /// The validator was removed from the validator set.
    Removed,
    /// The validator was banned from this era.
    Banned,
    /// The validator was excluded from proposing new blocks in this era.
    CannotPropose,
    /// The validator was seen to misbehave in this era.
    SeenAsFaulty,
}

/// A single change to a validator's status in the given era.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValidatorChangeInEra {
    /// The era in which the change occurred.
    pub era_id: EraId,
    /// The change in validator status.
    pub validator_change: ValidatorChange,
}

/// The changes in a validator's status.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValidatorChanges {
    /// The public key of the validator.
    pub public_key: PublicKey,
    /// The set of changes to the validator's status.
    pub status_changes: Vec<ValidatorChangeInEra>,
}

/// The `result` field of a successful JSON-RPC response to a `info_get_validator_changes` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetValidatorChangesResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The validators' status changes.
    pub changes: Vec<ValidatorChanges>,
}
