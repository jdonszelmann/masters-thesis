use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

#[derive(Debug, serde::Serialize)]
pub struct DidChangeConfigurationRegistrationOptions {
    pub section: Option<Union<String, Vec<String>>>,
}
