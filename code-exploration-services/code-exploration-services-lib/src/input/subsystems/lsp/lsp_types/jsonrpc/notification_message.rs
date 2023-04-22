use super::Value;

/// Notification Message
#[derive(Debug, serde::Serialize)]
pub struct NotificationMessage {
    pub jsonrpc: String,

    /// The method to be invoked.
    pub method: String,

    /// The notification's params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>
}
