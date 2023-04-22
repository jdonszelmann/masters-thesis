use super::{TextEdit, VersionedTextDocumentIdentifier};

/// Describes textual changes on a text document.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentEdit {
    /// The text document to change.
    pub text_document: VersionedTextDocumentIdentifier,

    /// The edits to be applied.
    pub edits: Vec<TextEdit>,
}

/// The TextDocumentEdit namespace provides helper function to create
/// an edit that manipulates a text document.
impl TextDocumentEdit {
    /// Creates a new `TextDocumentEdit`
    pub fn create(text_document: VersionedTextDocumentIdentifier, edits: Vec<TextEdit>) -> Self {
        TextDocumentEdit {
            text_document,
            edits,
        }
    }
}
