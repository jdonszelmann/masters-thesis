use super::FileEvent;

/// The watched files change notification's parameters.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeWatchedFilesParams {
    /// The actual file events.
    pub changes: Vec<FileEvent>,
}
