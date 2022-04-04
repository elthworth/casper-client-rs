use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use casper_types::{PublicKey, U512};

/// A validator's weight.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValidatorWeight {
    public_key: PublicKey,
    weight: U512,
}

impl ValidatorWeight {
    /// Returns the validator's public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Returns the validator's weight.
    pub fn weight(&self) -> U512 {
        self.weight
    }
}

impl Display for ValidatorWeight {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{{ validator {}, weight {} }}",
            self.public_key, self.weight
        )
    }
}
