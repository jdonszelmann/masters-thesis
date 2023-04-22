//! The show message notification is sent from a server to a client to ask
//! the client to display a particular message in the user interface.

use super::ShowMessageParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<ShowMessageParams, ()> = NotificationType {
    method: "window/showMessage",
    number_of_params: 1,
    __: None,
};
