use super::{TextDocumentEdit, TextEdit};
use std::collections::HashMap;

/// A workspace edit represents changes to many resources managed in the workspace. The edit
/// should either provide `changes` or `documentChanges`. If documentChanges are present
/// they are preferred over `changes` if the client can handle versioned document edits.
#[derive(Debug, serde::Serialize)]
pub struct WorkspaceEdit {
    /// Holds changes to existing resources.
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,

    /// An array of `TextDocumentEdit`s to express changes to n different text documents
    /// where each text document edit addresses a specific version of a text document.
    /// Whether a client supports versioned document edits is expressed via
    /// `WorkspaceClientCapabilites.workspaceEdit.documentChanges`.
    pub document_changes: Option<Vec<TextDocumentEdit>>,
}
