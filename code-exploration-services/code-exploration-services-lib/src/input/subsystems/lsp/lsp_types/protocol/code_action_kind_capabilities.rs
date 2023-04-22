use crate::input::subsystems::lsp::lsp_types::types::CodeActionKind;

/// The code action kind is support with the following value
/// set.
#[derive(Debug, serde::Serialize)]
pub struct CodeActionKindCapabilities {
    /// The code action kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Vec<CodeActionKind>,
}