//! The JSON-RPC request and response types.

pub mod common;
pub mod results;
pub(crate) mod v1_4_5;
pub use v1_4_5::{
    get_dictionary_item::DictionaryItemIdentifier, query_global_state::GlobalStateIdentifier,
};
