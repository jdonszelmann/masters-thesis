/// Defines which synchronization capabilities the client supports.
#[derive(Debug, serde::Serialize)]
pub struct SynchronizationCapabilities {
    /// Whether text document synchronization supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports sending will save notifications.
    pub will_save: Option<bool>,

    /// The client supports sending a will save request and
    /// waits for a response providing text edits which will
    /// be applied to the document before it is saved.
    pub will_save_wait_until: Option<bool>,

    /// The client supports did save notifications.
    pub did_save: Option<bool>,
}
