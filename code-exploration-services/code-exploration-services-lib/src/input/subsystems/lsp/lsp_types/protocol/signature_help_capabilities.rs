use super::SignatureInformationCapabilities;

/// Capabilities specific to the `textDocument/signatureHelp`
#[derive(Debug, serde::Serialize)]
pub struct SignatureHelpCapabilities {
    /// Whether signature help supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports the following `SignatureInformation`
    /// specific properties.
    pub signature_information: Option<SignatureInformationCapabilities>,
}
