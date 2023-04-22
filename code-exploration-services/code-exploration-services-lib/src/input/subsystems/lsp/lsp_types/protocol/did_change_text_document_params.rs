use crate::input::subsystems::lsp::lsp_types::types::{TextDocumentContentChangeEvent, VersionedTextDocumentIdentifier};

/// The change text document notification's parameters.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeTextDocumentParams {
    /// The document that did change. The version number points
    /// to the version after all provided content changes have
    /// been applied.
    pub text_document: VersionedTextDocumentIdentifier,

    /// The actual content changes. The content changes describe single state changes
    /// to the document. So if there are two content changes c1 and c2 for a document
    /// in state S10 then c1 move the document to S11 and c2 to S12.
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}
