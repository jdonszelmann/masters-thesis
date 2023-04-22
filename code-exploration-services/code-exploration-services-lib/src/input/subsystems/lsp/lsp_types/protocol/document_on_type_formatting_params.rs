use crate::input::subsystems::lsp::lsp_types::types::{FormattingOptions, Position, TextDocumentIdentifier};

#[derive(Debug, serde::Serialize)]
pub struct DocumentOnTypeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The position at which this request was send.
    pub position: Position,

    /// The character that has been typed.
    pub ch: String,

    /// The format options.
    pub options: FormattingOptions,
}
