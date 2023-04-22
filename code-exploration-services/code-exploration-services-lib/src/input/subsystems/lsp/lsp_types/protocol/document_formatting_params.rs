use crate::input::subsystems::lsp::lsp_types::types::{FormattingOptions, TextDocumentIdentifier};

#[derive(Debug, serde::Serialize)]
pub struct DocumentFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The format options
    pub options: FormattingOptions,
}
