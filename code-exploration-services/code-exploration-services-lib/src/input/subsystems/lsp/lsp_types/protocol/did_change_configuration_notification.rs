//! The configuration change notification is sent from the client to the server
//! when the client's configuration has changed. The notification contains
//! the changed configuration as defined by the language client.

use super::{DidChangeConfigurationParams, DidChangeConfigurationRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<
    DidChangeConfigurationParams,
    DidChangeConfigurationRegistrationOptions,
> = NotificationType {
    method: "workspace/didChangeConfiguration",
    number_of_params: 1,
    __: None,
};
