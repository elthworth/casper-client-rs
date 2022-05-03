//! The JSON-RPC request and response types.

pub mod common;
pub mod results;
/// RPCs provided by the v1.4.5 node.
pub(crate) mod v1_4_5;
/// RPCs provided by the v1.5.0 node.
pub(crate) mod v1_5_0;
pub use v1_5_0::{
    get_dictionary_item::DictionaryItemIdentifier, query_global_state::GlobalStateIdentifier,
};
