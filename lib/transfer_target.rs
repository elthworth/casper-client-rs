use casper_types::{account::AccountHash, PublicKey, URef};

/// The various types which can be used as the `target` runtime argument of a native transfer.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TransferTarget {
    /// A public key.
    PublicKey(PublicKey),
    /// An account hash.
    AccountHash(AccountHash),
    /// A URef.
    URef(URef),
}
