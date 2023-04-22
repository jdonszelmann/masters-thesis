/// Capabilities specific to the `workspace/executeCommand` request.
#[derive(Debug, serde::Serialize)]
pub struct ExecuteCommandCapabilities {
    /// Execute command supports dynamic registration.
    pub dynamic_registration: Option<bool>
}
