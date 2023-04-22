//! The document close notification is sent from the client to the server when
//! the document got closed in the client. The document's truth now exists where
//! the document's uri points to (e.g. if the document's uri is a file uri the
//! truth now exists on disk). As with the open notification the close notification
//! is about managing the document's content. Receiving a close notification
//! doesn't mean that the document was open in an editor before. A close
//! notification requires a previous open notification to be sent.

use super::{DidCloseTextDocumentParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::NotificationType;

pub const TYPE: NotificationType<DidCloseTextDocumentParams, TextDocumentRegistrationOptions> =
    NotificationType {
        method: "textDocument/didClose",
        number_of_params: 1,
        __: None,
    };
