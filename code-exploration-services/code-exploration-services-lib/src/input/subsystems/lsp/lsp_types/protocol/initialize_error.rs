/// If the protocol version provided by the client can't be handled by the server.
/// @deprecated This initialize error got replaced by client capabilities. There is
/// no version handshake in version 3.0x
pub const UNKNOWN_PROTOCOL_VERSION: i32 = 1;

/// The data type of the ResponseError if the
/// initialize request fails.
#[derive(Debug, serde::Serialize)]
pub struct InitializeError {
    /// Indicates whether the client execute the following retry logic:
    /// (1) show the message provided by the ResponseError to the user
    /// (2) user selects retry or cancel
    /// (3) if user selected retry the initialize method is sent again.
    pub retry: bool,
}
