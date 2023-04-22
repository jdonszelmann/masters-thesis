use super::{TextDocumentClientCapabilities, WorkspaceClientCapabilities};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// Defines the capabilities provided by the client.
#[derive(Debug, serde::Serialize)]
pub struct ClientCapabilities {
    /// Workspace specific client capabilities.
    pub workspace: Option<WorkspaceClientCapabilities>,

    /// Text document specific client capabilities.
    pub text_document: Option<TextDocumentClientCapabilities>,

    /// Experimental client capabilities.
    pub experimental: Option<Value>,
}
