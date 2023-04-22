use crate::input::subsystems::lsp::lsp_types::types::TextDocumentIdentifier;

/// Params for the Code Lens request.
#[derive(Debug, serde::Serialize)]
pub struct CodeLensParams {
    /// The document to request code lens for.
    pub text_document: TextDocumentIdentifier,
}
