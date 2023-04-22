use super::WatchKind;

#[derive(Debug, serde::Serialize)]
pub struct FileSystemWatcher {
    /// The  glob pattern to watch
    pub glob_pattern: String,

    /// The kind of events of interest. If omitted it defaults
    /// to WatchKind.Create | WatchKind.Change | WatchKind.Delete
    /// which is 7.
    pub kind: Option<WatchKind>,
}
