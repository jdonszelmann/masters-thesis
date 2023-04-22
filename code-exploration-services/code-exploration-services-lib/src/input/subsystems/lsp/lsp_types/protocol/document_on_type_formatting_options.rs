/// Format document on type options
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DocumentOnTypeFormattingOptions {
    /// A character on which formatting should be triggered, like `}`.
    pub first_trigger_character: String,

    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}
