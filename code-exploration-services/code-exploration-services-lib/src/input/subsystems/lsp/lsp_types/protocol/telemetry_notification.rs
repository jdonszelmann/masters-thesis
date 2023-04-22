//! The telemetry event notification is sent from the server to the client to ask
//! the client to log telemetry data.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::{NotificationType, Value};

pub const TYPE: NotificationType<Value, ()> = NotificationType {
    method: "telemetry/event",
    number_of_params: 1,
    __: None,
};
