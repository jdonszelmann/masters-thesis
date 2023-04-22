/// Capabilities specific to `WorkspaceEdit`s
#[derive(Debug, serde::Serialize)]
pub struct WorkspaceEditCapabilities {
    /// The client supports versioned document changes in `WorkspaceEdit`s
    pub document_changes: Option<bool>,
}
