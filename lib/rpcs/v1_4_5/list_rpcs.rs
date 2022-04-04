use schemars::{schema::Schema, Map};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use casper_types::ProtocolVersion;

pub(crate) const LIST_RPCS_METHOD: &str = "rpc.discover";

/// Contact information for the exposed API.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OpenRpcContactField {
    /// The identifying name of the organization.
    pub name: String,
    /// The URL pointing to the contact information.
    pub url: String,
}

/// License information for the exposed API.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OpenRpcLicenseField {
    /// The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API.
    pub url: String,
}

/// Metadata about the API.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OpenRpcInfoField {
    /// The version of this OpenRPC schema document.
    pub version: String,
    /// The title of the application.
    pub title: String,
    /// The description of the application.
    pub description: String,
    /// The contact information for the exposed API.
    pub contact: OpenRpcContactField,
    /// The license information for the exposed API.
    pub license: OpenRpcLicenseField,
}

/// An object representing a Server.
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct OpenRpcServerEntry {
    /// The name of the server.
    pub name: String,
    /// A URL to the server.
    pub url: String,
}

/// A parameter that is applicable to a method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SchemaParam {
    /// The name of the parameter.
    pub name: String,
    /// The schema describing the parameter.
    pub schema: Schema,
    /// Whether the parameter is required or not.
    pub required: bool,
}

/// The description of the result returned by the method.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ResponseResult {
    /// Name of the response result.
    pub name: String,
    /// The schema describing the response result.
    pub schema: Schema,
}

/// An example request parameter.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ExampleParam {
    /// Canonical name of the example parameter.
    pub name: String,
    /// Embedded literal example parameter.
    pub value: Value,
}

/// An example result.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ExampleResult {
    /// Canonical name of the example result.
    pub name: String,
    /// Embedded literal example result.
    pub value: Value,
}

/// An example pair of request parameters and response result.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Example {
    /// Name for the example pairing.
    pub name: String,
    /// Example parameters.
    pub params: Vec<ExampleParam>,
    /// Example result.
    pub result: ExampleResult,
}

/// Describes the interface for the given method name.  The method name is used as the `method`
/// field of the JSON-RPC body.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Method {
    /// The method name.
    pub name: String,
    /// A short summary of what the method does.
    pub summary: String,
    /// A list of parameters that are applicable for this method.
    pub params: Vec<SchemaParam>,
    /// The description of the result returned by the method.
    pub result: ResponseResult,
    /// An array of examples.
    pub examples: Vec<Example>,
}

/// The schema components.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Components {
    /// The map of component schemas.
    pub schemas: Map<String, Schema>,
}

/// The main schema for the casper node's RPC server, compliant with
/// [the OpenRPC Specification](https://spec.open-rpc.org).
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct OpenRpcSchema {
    /// The OpenRPC Standard version.
    pub openrpc: String,
    /// The OpenRPC info field.
    pub info: OpenRpcInfoField,
    /// Available servers for the `rpc.discovery` method.
    pub servers: Vec<OpenRpcServerEntry>,
    /// The available RPC methods.
    pub methods: Vec<Method>,
    /// The schema components.
    pub components: Components,
}

/// The `result` field of a successful JSON-RPC response to a `rpc.discover` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListRpcsResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// Hard coded name: "OpenRPC Schema".
    pub name: String,
    /// The list of supported RPCs described in OpenRPC schema format.
    pub schema: OpenRpcSchema,
}
