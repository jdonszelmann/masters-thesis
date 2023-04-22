use super::DocumentSelector;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// Format document on type options
#[derive(Debug, serde::Serialize)]
pub struct DocumentOnTypeFormattingRegistrationOptions {
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Nullable<DocumentSelector>,

    /// A character on which formatting should be triggered, like `}`.
    pub first_trigger_character: String,

    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}
