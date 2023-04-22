use super::WorkspaceFolder;

/// The workspace folder change event.
pub struct WorkspaceFoldersChangeEvent {
    /// The array of added workspace folders
    pub added: Vec<WorkspaceFolder>,

    /// The array of the removed workspace folders
    pub removed: Vec<WorkspaceFolder>,
}
