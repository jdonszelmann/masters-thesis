use super::FileChangeType;

/// An event describing a file change.
#[derive(Debug, serde::Serialize)]
pub struct FileEvent {
    /// The file's uri.
    pub uri: String,

    /// The change type.
    pub hange_type: FileChangeType,
}
