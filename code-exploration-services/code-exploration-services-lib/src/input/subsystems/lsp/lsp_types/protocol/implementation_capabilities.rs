/// Capabilities specific to the `textDocument/implementation`
#[derive(Debug, serde::Serialize)]
pub struct ImplementationCapabilities {
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}