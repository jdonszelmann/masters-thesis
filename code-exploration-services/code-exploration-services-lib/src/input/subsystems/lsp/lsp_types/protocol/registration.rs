use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// General parameters to to register for an notification or to register a provider.
#[derive(Debug, serde::Serialize)]
pub struct Registration {
    /// The id used to register the request. The id can be used to deregister
    /// the request again.
    pub id: String,

    /// The method to register for.
    pub method: String,

    /// Options necessary for the registration.
    pub register_options: Option<Value>,
}
