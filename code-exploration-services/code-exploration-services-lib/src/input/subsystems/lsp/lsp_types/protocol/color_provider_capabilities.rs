/// Capabilities specific to the colorProvider
#[derive(Debug, serde::Serialize)]
pub struct ColorProviderCapabilities {
    
     /// Whether colorProvider supports dynamic registration. If this is set to `true`
     /// the client supports the new `(ColorProviderOptions & TextDocumentRegistrationOptions & StaticRegistrationOptions)`
     /// return value for the corresponding server capability as well.
    
    pub dynamic_registration: Option<bool>,
}
