use crate::input::subsystems::lsp::lsp_types::types::{FormattingOptions, Range, TextDocumentIdentifier};

#[derive(Debug, serde::Serialize)]
pub struct DocumentRangeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The range to format
    pub range: Range,

    /// The format options
    pub options: FormattingOptions,
}
