use super::CompletionContext;
use crate::input::subsystems::lsp::lsp_types::types::{Position, TextDocumentIdentifier};

/// Completion parametersS
#[derive(Debug, serde::Serialize)]
pub struct CompletionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    /// The completion context. This is only available it the client specifies
    /// to send this using `ClientCapabilities.textDocument.completion.contextSupport === true`
    pub context: Option<CompletionContext>,
}
