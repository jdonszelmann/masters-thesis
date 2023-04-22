use crate::input::subsystems::lsp::lsp_types::types::MarkupKind;

/// The client supports the following `SignatureInformation`
/// specific properties.
#[derive(Debug, serde::Serialize)]
pub struct SignatureInformationCapabilities {
    /// Client supports the follow content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,
}
