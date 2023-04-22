/// Capabilities specific to the `textDocument/codeLens`
#[derive(Debug, serde::Serialize)]
pub struct CodeLensCapabilities {
    /// Whether code lens supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
