/// Capabilities specific to the `textDocument/formatting`
#[derive(Debug, serde::Serialize)]
pub struct FormattingCapabilities {
    /// Whether formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
