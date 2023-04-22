use super::CodeActionKindCapabilities;

/// The client support code action literals as a valid
/// response of the `textDocument/codeAction` request.
#[derive(Debug, serde::Serialize)]
pub struct CodeActionLiteralSupportCapabilities {
    /// The code action kind is support with the following value
    /// set.
    pub code_action_kind: CodeActionKindCapabilities,
}