/// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeWatchedFilesCapabilities {
    /// Did change watched files notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
