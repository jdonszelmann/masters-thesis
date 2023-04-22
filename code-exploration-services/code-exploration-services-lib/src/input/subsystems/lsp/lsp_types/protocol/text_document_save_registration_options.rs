#[derive(Debug, serde::Serialize)]
pub struct TextDocumentSaveRegistrationOptions {
    /// The client is supposed to include the content on save.
    pub include_text: Option<bool>,
}
