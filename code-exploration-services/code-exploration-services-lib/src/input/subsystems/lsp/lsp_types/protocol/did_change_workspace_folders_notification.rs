//! The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server when the workspace
//! folder configuration changes.

use super::DidChangeWorkspaceFoldersParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<DidChangeWorkspaceFoldersParams, ()> = NotificationType {
    method: "workspace/didChangeWorkspaceFolders",
    number_of_params: 1,
    __: None,
};
