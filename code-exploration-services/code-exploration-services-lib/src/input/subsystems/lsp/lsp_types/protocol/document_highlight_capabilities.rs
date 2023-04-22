/// Capabilities specific to the `textDocument/documentHighlight`
#[derive(Debug, serde::Serialize)]
pub struct DocumentHighlightCapabilities {
    /// Whether document highlight supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
