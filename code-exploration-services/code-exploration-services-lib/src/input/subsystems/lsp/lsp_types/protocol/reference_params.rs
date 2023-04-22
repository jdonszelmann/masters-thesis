use crate::input::subsystems::lsp::lsp_types::types::{Position, ReferenceContext, TextDocumentIdentifier};

/// Parameters for a [ReferencesRequest](#ReferencesRequest).
#[derive(Debug, serde::Serialize)]
pub struct ReferenceParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    pub context: ReferenceContext,
}
