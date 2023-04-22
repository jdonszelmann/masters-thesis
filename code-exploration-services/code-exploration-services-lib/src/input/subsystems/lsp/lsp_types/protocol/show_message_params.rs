use super::MessageType;

/// The parameters of a notification message.
#[derive(Debug, serde::Serialize)]
pub struct ShowMessageParams {
    /// The message type. See {@link MessageType}
    pub message_type: MessageType,

    /// The actual message
    pub message: String,
}
