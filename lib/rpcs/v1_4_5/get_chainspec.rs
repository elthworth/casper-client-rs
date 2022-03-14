use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use casper_types::{bytesrepr::Bytes, ProtocolVersion};

pub(crate) const GET_CHAINSPEC_METHOD: &str = "info_get_chainspec";

/// The raw bytes of the chainspec.toml, genesis accounts.toml, and global_state.toml files.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct ChainspecRawBytes {
    /// The raw bytes of the chainspec.toml file.
    pub chainspec_bytes: Bytes,
    /// The raw bytes of the genesis accounts.toml file, if it exists.
    pub maybe_genesis_accounts_bytes: Option<Bytes>,
    /// The raw bytes of the global_state.toml file, if it exists.
    pub maybe_global_state_bytes: Option<Bytes>,
}

impl Display for ChainspecRawBytes {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            String::from_utf8_lossy(&self.chainspec_bytes)
        )?;
        if let Some(genesis_accounts_bytes) = &self.maybe_genesis_accounts_bytes {
            write!(
                formatter,
                "{}",
                String::from_utf8_lossy(genesis_accounts_bytes)
            )?;
        }
        if let Some(global_state_bytes) = &self.maybe_global_state_bytes {
            write!(formatter, "{}", String::from_utf8_lossy(global_state_bytes))?;
        }
        Ok(())
    }
}

/// The `result` field of a successful JSON-RPC response to a `info_get_chainspec` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetChainspecResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The chainspec file bytes.
    pub chainspec_bytes: ChainspecRawBytes,
}
