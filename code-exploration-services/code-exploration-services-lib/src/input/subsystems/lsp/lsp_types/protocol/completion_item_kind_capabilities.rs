use crate::input::subsystems::lsp::lsp_types::types::CompletionItemKind;

#[derive(Debug, serde::Serialize)]
pub struct CompletionItemKindCapabilities {
    /// The completion item kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    ///
    /// If this property is not present the client only supports
    /// the completion items kinds from `Text` to `Reference` as defined in
    /// the initial version of the protocol.
    pub value_set: Option<Vec<CompletionItemKind>>,
}
