use super::{SaveOptions, TextDocumentSyncKind};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TextDocumentSyncOptions {
    /// Open and close notifications are sent to the server.
    pub open_close: Option<bool>,

    /// Change notifications are sent to the server. See TextDocumentSyncKind.None, TextDocumentSyncKind.Full
    /// and TextDocumentSyncKind.Incremental.
    pub change: Option<TextDocumentSyncKind>,

    /// Will save notifications are sent to the server.
    pub will_save: Option<bool>,

    /// Will save wait until requests are sent to the server.
    pub will_save_wait_until: Option<bool>,

    /// Save notifications are sent to the server.
    pub save: Option<SaveOptions>,
}
