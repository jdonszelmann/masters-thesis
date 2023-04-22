use super::DocumentSelector;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// Completion registration options.
#[derive(Debug, serde::Serialize)]
pub struct CompletionRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Nullable<DocumentSelector>,

    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    /// The characters that trigger completion automatically.
    pub trigger_characters: Option<Vec<String>>,
}
