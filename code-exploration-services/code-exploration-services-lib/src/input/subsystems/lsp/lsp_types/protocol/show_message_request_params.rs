use super::{MessageActionItem, MessageType};

#[derive(Debug, serde::Serialize)]
pub struct ShowMessageRequestParams {
    /// The message type. See {@link MessageType}
    pub message_type: MessageType,

    /// The actual message
    pub message: String,

    /// The message action items to present.
    pub actions: Option<Vec<MessageActionItem>>,
}
