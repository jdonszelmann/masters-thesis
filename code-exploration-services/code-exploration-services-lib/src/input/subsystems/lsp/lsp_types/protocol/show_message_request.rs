//! The show message request is sent from the server to the client to show a message
//! and a set of options actions to the user.

use super::{ShowMessageRequestParams, MessageActionItem };
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{RequestType, Nullable};

pub const TYPE: RequestType<ShowMessageRequestParams, Nullable<MessageActionItem>, (), ()> = RequestType {
    method: "window/showMessageRequest",
    number_of_params: 1,
    __: None,
};
