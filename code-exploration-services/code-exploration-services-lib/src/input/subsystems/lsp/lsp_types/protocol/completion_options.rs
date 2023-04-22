/// Completion options.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionOptions {
    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    /// The characters that trigger completion automatically.
    pub trigger_characters: Option<Vec<String>>,
}
