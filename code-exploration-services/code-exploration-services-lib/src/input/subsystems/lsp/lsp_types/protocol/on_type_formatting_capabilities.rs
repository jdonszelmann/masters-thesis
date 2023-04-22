/// Capabilities specific to the `textDocument/onTypeFormatting`
#[derive(Debug, serde::Serialize)]
pub struct OnTypeFormattingCapabilities {
    /// Whether on type formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
