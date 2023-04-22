use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

#[derive(Debug, serde::Serialize)]
pub struct ExecuteCommandParams {
    /// The identifier of the actual command handler.
    pub command: String,

    /// Arguments that the command should be invoked with.
    pub arguments: Option<Vec<Value>>,
}
