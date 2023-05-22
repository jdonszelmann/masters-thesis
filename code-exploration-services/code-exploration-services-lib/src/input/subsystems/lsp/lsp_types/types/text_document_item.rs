/// An item to transfer a text document from the client to the
/// server.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentItem {
    /// The text document's uri.
    pub uri: String,

    /// The text document's language identifier
    #[serde(rename="languageId")]
    pub language_id: String,

    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    pub version: i32,

    /// The content of the opened text document.
    pub text: String,
}

/// The TextDocumentItem namespace provides helper functions to work with
/// [TextDocumentItem](#TextDocumentItem) literals.
impl TextDocumentItem {
    /// Creates a new TextDocumentItem literal.
    /// @param uri The document's uri.
    /// @param languageId The document's language identifier.
    /// @param version The document's version number.
    /// @param text The document's text.
    pub fn create(uri: String, language_id: String, version: i32, text: String) -> Self {
        TextDocumentItem { uri, language_id, version, text }
    }
}
