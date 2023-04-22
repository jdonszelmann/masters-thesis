use crate::input::subsystems::lsp::lsp_types::types::MarkupKind;

/// Capabilities specific to the `textDocument/hover`
#[derive(Debug, serde::Serialize)]
pub struct HoverCapabilities {
    /// Whether hover supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Client supports the follow content formats for the content
    /// property. The order describes the preferred format of the client.
    pub content_format: Option<Vec<MarkupKind>>,
}
