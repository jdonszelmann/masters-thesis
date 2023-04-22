use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// The parameters of a change configuration notification.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeConfigurationParams {
    /// The actual changed settings
    pub settings: Value,
}
