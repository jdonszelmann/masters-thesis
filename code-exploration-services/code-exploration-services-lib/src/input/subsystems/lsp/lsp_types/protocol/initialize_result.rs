use super::ServerCapabilities;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;
use std::collections::HashMap;

/// The result returned from an initialize request.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    /// Custom initialization results.
    #[serde(default)]
    pub results: HashMap<String, Value>,
}
