use crate::input::subsystems::lsp::lsp_types::types::{Position, TextDocumentIdentifier};

#[derive(Debug, serde::Serialize)]
pub struct RenameParams {
    /// The document to rename.
    pub text_document: TextDocumentIdentifier,

    /// The position at which this request was sent.
    pub position: Position,

    /// The new name of the symbol. If the given name is not valid the
    /// request must return a [ResponseError](#ResponseError) with an
    /// appropriate message set.
    pub new_name: String,
}
