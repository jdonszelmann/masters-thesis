use crate::input::subsystems::lsp::lsp_types::types::TextDocumentIdentifier;

#[derive(Debug, serde::Serialize)]
pub struct DocumentLinkParams {
    /// The document to provide document links for.
    pub text_document: TextDocumentIdentifier,
}
