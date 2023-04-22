//! The document change notification is sent from the client to the server to signal
//! changes to a text document.

use super::{DidChangeTextDocumentParams, TextDocumentChangeRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<DidChangeTextDocumentParams, TextDocumentChangeRegistrationOptions> =
    NotificationType {
        method: "textDocument/didChange",
        number_of_params: 1,
        __: None,
    };
