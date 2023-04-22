use super::{
    DidChangeConfigurationCapabilities, DidChangeWatchedFilesCapabilities,
    ExecuteCommandCapabilities, SymbolCapabilities, WorkspaceEditCapabilities,
};

/// Workspace specific client capabilities.
#[derive(Debug, serde::Serialize)]
pub struct WorkspaceClientCapabilities {
    /// The client supports applying batch edits
    /// to the workspace by supporting the request
    /// 'workspace/applyEdit'
    pub apply_edit: Option<bool>,

    /// Capabilities specific to `WorkspaceEdit`s
    pub workspace_edit: Option<WorkspaceEditCapabilities>,

    /// Capabilities specific to the `workspace/didChangeConfiguration` notification.
    pub did_change_configuration: Option<DidChangeConfigurationCapabilities>,

    /// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
    pub did_change_watched_files: Option<DidChangeWatchedFilesCapabilities>,

    /// Capabilities specific to the `workspace/symbol` request.
    pub symbol: Option<SymbolCapabilities>,

    /// Capabilities specific to the `workspace/executeCommand` request.
    pub execute_command: Option<ExecuteCommandCapabilities>,

    /// The client has support for workspace folders.
    ///
    /// Since 3.6.0
    pub workspace_folders: Option<bool>,

    /// The client supports `workspace/configuration` requests.
    ///
    /// Since 3.6.0
    pub configuration: Option<bool>,
}
