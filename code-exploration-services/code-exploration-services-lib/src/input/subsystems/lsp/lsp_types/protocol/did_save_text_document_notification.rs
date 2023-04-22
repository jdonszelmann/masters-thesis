//! The document save notification is sent from the client to the server when
//! the document got saved in the client.

use super::{DidSaveTextDocumentParams, TextDocumentSaveRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<DidSaveTextDocumentParams, TextDocumentSaveRegistrationOptions> =
    NotificationType {
        method: "textDocument/didSave",
        number_of_params: 1,
        __: None,
    };
