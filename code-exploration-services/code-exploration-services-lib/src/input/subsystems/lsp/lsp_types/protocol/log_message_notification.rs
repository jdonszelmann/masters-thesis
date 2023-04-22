//! The log message notification is sent from the server to the client to ask
//! the client to log a particular message.

use super::LogMessageParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<LogMessageParams, ()> = NotificationType {
    method: "window/logMessage",
    number_of_params: 1,
    __: None,
};
