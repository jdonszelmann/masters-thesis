use crate::input::subsystems::lsp::lsp_types::types::{Position, TextDocumentIdentifier};

/// A parameter literal used in requests to pass a text document and a position inside that
/// document.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentPositionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,
}
