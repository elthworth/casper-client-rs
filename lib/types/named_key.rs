use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use casper_types::{Key, KeyFromStrError};

#[cfg(doc)]
use crate::types::{Account, Contract};

/// A named key.  An [`Account`] or [`Contract`] may store a collection of named keys in global
/// state.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct NamedKey {
    name: String,
    key: String,
}

impl NamedKey {
    /// Returns the name under which the key is held.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the key.
    pub fn key(&self) -> Result<Key, KeyFromStrError> {
        Key::from_formatted_str(&self.key)
    }
}

impl Display for NamedKey {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}: {}", self.name, self.key)
    }
}
