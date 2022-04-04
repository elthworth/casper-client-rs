use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// An identifier for a JSON-RPC, provided by the client and returned in the response.
///
/// The JSON-RPC spec allows for NULL to be used as the RPC-ID to imply a client notification, but
/// the Casper node JSON-RPC server does not provide any such notification endpoints, so this is
/// not supported here.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    /// A numeric identifier.
    Number(i64),
    /// A text identifier.
    String(String),
}

impl From<i64> for Id {
    fn from(value: i64) -> Self {
        Id::Number(value)
    }
}

impl From<String> for Id {
    fn from(value: String) -> Self {
        Id::String(value)
    }
}

impl From<&Id> for jsonrpc_lite::Id {
    fn from(rpc_id: &Id) -> Self {
        match rpc_id {
            Id::String(string_id) => jsonrpc_lite::Id::Str(string_id.clone()),
            Id::Number(number_id) => jsonrpc_lite::Id::Num(*number_id),
        }
    }
}

impl Display for Id {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Id::Number(number) => write!(formatter, "{}", number),
            Id::String(string) => write!(formatter, "{}", string),
        }
    }
}
