/// A response returned from the apply workspace edit request.
#[derive(Debug, serde::Serialize)]
pub struct ApplyWorkspaceEditResponse {
    /// Indicates whether the edit was applied or not.
    pub applied: bool,
}
