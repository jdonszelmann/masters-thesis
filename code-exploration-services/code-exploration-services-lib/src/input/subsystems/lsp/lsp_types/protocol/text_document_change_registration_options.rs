use super::TextDocumentSyncKind;

/// Describe options to be used when registered for text document change events.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentChangeRegistrationOptions {
    /// How documents are synced to the server.
    pub sync_kind: TextDocumentSyncKind,
}
