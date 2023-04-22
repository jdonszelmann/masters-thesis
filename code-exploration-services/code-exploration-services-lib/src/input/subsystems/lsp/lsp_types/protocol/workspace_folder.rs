#[derive(Debug, serde::Serialize)]
pub struct WorkspaceFolder {
    /// The associated URI for this workspace folder.
    pub uri: String,

    /// The name of the workspace folder. Defaults to the
    /// uri's basename.
    pub name: String,
}
