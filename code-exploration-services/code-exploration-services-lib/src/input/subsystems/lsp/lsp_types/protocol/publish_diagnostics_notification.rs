//! Diagnostics notification are sent from the server to the client to signal
//! results of validation runs.

use super::PublishDiagnosticsParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<PublishDiagnosticsParams, ()> = NotificationType {
    method: "textDocument/publishDiagnostics",
    number_of_params: 1,
    __: None,
};
