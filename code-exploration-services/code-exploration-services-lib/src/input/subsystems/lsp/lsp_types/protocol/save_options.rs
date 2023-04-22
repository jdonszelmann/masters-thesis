/// Save options.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SaveOptions {
    /// The client is supposed to include the content on save.
    pub include_text: Option<bool>,
}
