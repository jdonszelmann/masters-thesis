use super::MessageType;

/// The log message parameters.
#[derive(Debug, serde::Serialize)]
pub struct LogMessageParams {
    /// The message type. See {@link MessageType}
    pub message_type: MessageType,

    /// The actual message
    pub message: String,
}
