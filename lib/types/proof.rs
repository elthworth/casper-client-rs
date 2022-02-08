use serde::{Deserialize, Serialize};

use casper_types::crypto::{PublicKey, Signature};

#[cfg(doc)]
use crate::types::Block;

/// A pair of public key and signature, representing proof of having signed a given piece of data,
/// generally a [`Block`].
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Proof {
    public_key: PublicKey,
    signature: Signature,
}

impl Proof {
    /// Returns the public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Returns the signature.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}
