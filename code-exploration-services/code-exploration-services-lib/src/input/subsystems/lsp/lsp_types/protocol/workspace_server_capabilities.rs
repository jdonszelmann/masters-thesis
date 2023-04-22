use super::WorkspaceFoldersOptions;

/// Workspace specific server capabilities
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceServerCapabilities {
    /// The server supports workspace folder.
    ///
    /// Since 3.6.0
    pub workspace_folders: Option<WorkspaceFoldersOptions>,
}
