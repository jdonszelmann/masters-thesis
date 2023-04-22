//! The document open notification is sent from the client to the server to signal
//! newly opened text documents. The document's truth is now managed by the client
//! and the server must not try to read the document's truth using the document's
//! uri. Open in this sense means it is managed by the client. It doesn't necessarily
//! mean that its content is presented in an editor. An open notification must not
//! be sent more than once without a corresponding close notification send before.
//! This means open and close notification must be balanced and the max open count
//! is one.

use super::{DidOpenTextDocumentParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<DidOpenTextDocumentParams, TextDocumentRegistrationOptions> =
    NotificationType {
        method: "textDocument/didOpen",
        number_of_params: 1,
        __: None,
    };