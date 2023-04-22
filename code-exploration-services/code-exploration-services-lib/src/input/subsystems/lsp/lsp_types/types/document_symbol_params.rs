use super::TextDocumentIdentifier;

/// Parameters for a [DocumentSymbolRequest](#DocumentSymbolRequest).
#[derive(Debug, serde::Serialize)]
pub struct DocumentSymbolParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,
}
