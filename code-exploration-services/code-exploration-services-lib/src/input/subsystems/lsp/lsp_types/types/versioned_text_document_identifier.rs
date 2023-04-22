use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// An identifier to denote a specific version of a text document.
#[derive(Debug, serde::Serialize)]
pub struct VersionedTextDocumentIdentifier {
    /// The text document's uri.
    pub uri: String,
    
    /// The version number of this document. If a versioned text document identifier
    /// is sent from the server to the client and the file is not open in the editor
    /// (the server has not received an open notification before) the server can send
    /// `null` to indicate that the version is unknown and the content on disk is the
    /// truth (as speced with document content ownership).
    pub version: Nullable<i32>,
}

/// The VersionedTextDocumentIdentifier namespace provides helper functions to work with
/// [VersionedTextDocumentIdentifier](#VersionedTextDocumentIdentifier) literals.
impl VersionedTextDocumentIdentifier {
    /// Creates a new VersionedTextDocumentIdentifier literal.
    /// @param uri The document's uri.
    /// @param uri The document's text.
    pub fn create(uri: String, version: Nullable<i32>) -> Self {
        VersionedTextDocumentIdentifier { uri, version }
    }
}
