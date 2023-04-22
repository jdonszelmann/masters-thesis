//! A document will save notification is sent from the client to the server before
//! the document is actually saved.

use super::{WillSaveTextDocumentParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<WillSaveTextDocumentParams, TextDocumentRegistrationOptions> =
    NotificationType {
        method: "textDocument/willSave",
        number_of_params: 1,
        __: None,
    };
