use crate::input::subsystems::lsp::lsp_types::types::WorkspaceEdit;

/// The parameters passed via a apply workspace edit request.
#[derive(Debug, serde::Serialize)]
pub struct ApplyWorkspaceEditParams {
    /// An optional label of the workspace edit. This label is
    /// presented in the user interface for example on an undo
    /// stack to undo the workspace edit.
    pub label: Option<String>,

    /// The edits to apply.
    pub edit: WorkspaceEdit,
}
