use super::DocumentSelector;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// General text document registration options.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TextDocumentRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Nullable<DocumentSelector>,
}
