use super::DocumentSelector;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// Document link registration options
#[derive(Debug, serde::Serialize)]
pub struct DocumentLinkRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Nullable<DocumentSelector>,

    /// Document links have a resolve provider as well.
    pub resolve_provider: Option<bool>,
}
