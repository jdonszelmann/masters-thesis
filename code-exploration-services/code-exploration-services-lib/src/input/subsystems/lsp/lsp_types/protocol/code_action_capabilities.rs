use super::CodeActionLiteralSupportCapabilities;

/// Capabilities specific to the `textDocument/codeAction`
#[derive(Debug, serde::Serialize)]
pub struct CodeActionCapabilities {
    /// Whether code action supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client support code action literals as a valid
    /// response of the `textDocument/codeAction` request.
    pub code_action_literal_support: Option<CodeActionLiteralSupportCapabilities>,
}
