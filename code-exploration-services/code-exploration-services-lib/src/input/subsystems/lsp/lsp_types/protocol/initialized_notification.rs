//! The intialized notification is sent from the client to the
//! server after the client is fully initialized and the server
//! is allowed to send requests from the server to the client.

use super::InitializedParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<InitializedParams, ()> = NotificationType {
    method: "initialized",
    number_of_params: 1,
    __: None,
};
