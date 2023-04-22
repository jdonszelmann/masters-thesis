use crate::input::subsystems::lsp::lsp_types::types::{CodeActionContext, Range, TextDocumentIdentifier};

/// Params for the CodeActionRequest
#[derive(Debug, serde::Serialize)]
pub struct CodeActionParams {
    /// The document in which the command was invoked.
    pub text_document: TextDocumentIdentifier,

    /// The range for which the command was invoked.
    pub range: Range,

    /// Context carrying additional information.
    pub context: CodeActionContext,
}
