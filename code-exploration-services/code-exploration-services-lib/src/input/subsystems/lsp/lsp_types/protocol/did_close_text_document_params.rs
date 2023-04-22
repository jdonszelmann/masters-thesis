use crate::input::subsystems::lsp::lsp_types::types::TextDocumentIdentifier;

/// The parameters send in a close text document notification
#[derive(Debug, serde::Serialize)]
pub struct DidCloseTextDocumentParams {
    /// The document that was closed.
    pub text_document: TextDocumentIdentifier,
}
