//! The exit event is sent from the client to the server to
//! ask the server to exit its process.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType0;

pub const TYPE: NotificationType0<()> = NotificationType0 {
    method: "exit",
    number_of_params: 1,
    __: None,
};
