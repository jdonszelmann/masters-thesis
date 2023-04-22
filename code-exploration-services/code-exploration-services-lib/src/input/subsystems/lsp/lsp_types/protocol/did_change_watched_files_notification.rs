//! The watched files notification is sent from the client to the server when
//! the client detects changes to file watched by the language client.

use super::{DidChangeWatchedFilesParams, DidChangeWatchedFilesRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<
    DidChangeWatchedFilesParams,
    DidChangeWatchedFilesRegistrationOptions,
> = NotificationType {
    method: "workspace/didChangeWatchedFiles",
    number_of_params: 1,
    __: None,
};
