use super::DocumentSelector;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// Code Lens registration options.
#[derive(Debug, serde::Serialize)]
pub struct CodeLensRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Nullable<DocumentSelector>,

    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,
}
