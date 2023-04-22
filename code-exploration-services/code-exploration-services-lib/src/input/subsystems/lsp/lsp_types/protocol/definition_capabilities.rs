/// Capabilities specific to the `textDocument/definition`
#[derive(Debug, serde::Serialize)]
pub struct DefinitionCapabilities {
    /// Whether definition supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
