//! Cryptographic types and functions.

mod asymmetric_key;
mod asymmetric_key_ext;
/// Cryptographic error.
pub mod error;

pub(crate) use asymmetric_key::sign;
pub use asymmetric_key_ext::AsymmetricKeyExt;
pub use error::Error as CryptoError;
