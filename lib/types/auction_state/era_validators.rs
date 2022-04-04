use std::fmt::{self, Display, Formatter};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use casper_types::EraId;

use super::ValidatorWeight;

/// The validators and their weights for the given era.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EraValidators {
    era_id: EraId,
    validator_weights: Vec<ValidatorWeight>,
}

impl EraValidators {
    /// Returns the era ID.
    pub fn era_id(&self) -> EraId {
        self.era_id
    }

    /// Returns the validators and their weights.
    pub fn validator_weights(&self) -> impl Iterator<Item = &ValidatorWeight> {
        self.validator_weights.iter()
    }
}

impl Display for EraValidators {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{{ {}, {{{}}} }}",
            self.era_id,
            self.validator_weights().format(", ")
        )
    }
}
