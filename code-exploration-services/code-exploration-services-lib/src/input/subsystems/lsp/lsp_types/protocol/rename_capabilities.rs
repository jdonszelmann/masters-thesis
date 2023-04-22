/// Capabilities specific to the `textDocument/rename`
#[derive(Debug, serde::Serialize)]
pub struct RenameCapabilities {
    /// Whether rename supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
