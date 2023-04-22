#[derive(Debug, serde::Serialize)]
pub struct MessageActionItem {
    /// A short title like 'Retry', 'Open Log' etc.
    pub title: String,
}
