use crate::input::subsystems::lsp::lsp_types::types::TextDocumentItem;

/// The parameters send in a open text document notification
#[derive(Debug, serde::Serialize)]
#[serde(rename_all="camelCase")]
pub struct DidOpenTextDocumentParams {
    /// The document that was opened.
    pub text_document: TextDocumentItem,
}
