/// Signature help registration options.
#[derive(Debug, serde::Serialize)]
pub struct SignatureHelpRegistrationOptions {
    /// The characters that trigger signature help
    /// automatically.
    pub trigger_characters: Option<Vec<String>>,
}
