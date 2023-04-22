/// A literal to identify a text document in the client.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentIdentifier {
    /// The text document's uri.
    pub uri: String,
}

/// The TextDocumentIdentifier namespace provides helper functions to work with
/// [TextDocumentIdentifier](#TextDocumentIdentifier) literals.
impl TextDocumentIdentifier {
    /// Creates a new TextDocumentIdentifier literal.
    /// @param uri The document's uri.
    pub fn create(uri: String) -> Self {
        TextDocumentIdentifier { uri }
    }
}
