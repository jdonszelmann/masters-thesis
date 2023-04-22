/// Capabilities specific to the `textDocument/documentLink`
#[derive(Debug, serde::Serialize)]
pub struct DocumentLinkCapabilities {
    /// Whether document link supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
