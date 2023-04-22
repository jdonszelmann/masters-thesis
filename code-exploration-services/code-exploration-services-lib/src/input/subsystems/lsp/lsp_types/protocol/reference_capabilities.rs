/// Capabilities specific to the `textDocument/references`
#[derive(Debug, serde::Serialize)]
pub struct ReferencesCapabilities {
    /// Whether references supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
