use crate::input::subsystems::lsp::lsp_types::types::VersionedTextDocumentIdentifier;

/// The parameters send in a save text document notification
#[derive(Debug, serde::Serialize)]
pub struct DidSaveTextDocumentParams {
    /// The document that was closed.
    pub text_document: VersionedTextDocumentIdentifier,

    /// Optional the content when saved. Depends on the includeText value
    /// when the save notification was requested.
    pub text: Option<String>,
}
