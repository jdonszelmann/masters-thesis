/// Capabilities specific to the `textDocument/rangeFormatting`
#[derive(Debug, serde::Serialize)]
pub struct RangeFormattingCapabilities {
    /// Whether range formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
