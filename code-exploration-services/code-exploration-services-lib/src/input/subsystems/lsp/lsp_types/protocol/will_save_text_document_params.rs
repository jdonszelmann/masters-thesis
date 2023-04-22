use crate::input::subsystems::lsp::lsp_types::types::{TextDocumentIdentifier, TextDocumentSaveReason};

/// The parameters send in a will save text document notification.
#[derive(Debug, serde::Serialize)]
pub struct WillSaveTextDocumentParams {
    /// The document that will be saved.
    pub text_document: TextDocumentIdentifier,

    /// The 'TextDocumentSaveReason'.
    pub reason: TextDocumentSaveReason,
}
