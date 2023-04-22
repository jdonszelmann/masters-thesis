/// Signature help options.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SignatureHelpOptions {
    /// The characters that trigger signature help
    /// automatically.
    pub trigger_characters: Option<Vec<String>>,
}
