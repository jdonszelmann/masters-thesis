/// Capabilities specific to the `workspace/didChangeConfiguration` notification.
#[derive(Debug, serde::Serialize)]
pub struct DidChangeConfigurationCapabilities {
    /// Did change configuration notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
