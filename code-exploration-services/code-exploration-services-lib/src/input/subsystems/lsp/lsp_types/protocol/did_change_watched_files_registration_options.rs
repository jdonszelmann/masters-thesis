use super::FileSystemWatcher;

/// Describe options to be used when registered for text document change events.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeWatchedFilesRegistrationOptions {
    /// The watchers to register.
    pub watchers: Vec<FileSystemWatcher>,
}
