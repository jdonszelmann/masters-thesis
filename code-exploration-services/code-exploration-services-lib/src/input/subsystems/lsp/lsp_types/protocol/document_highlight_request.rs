//! Request to resolve a [DocumentHighlight](#DocumentHighlight) for a given
//! text document position. The request's parameter is of type [TextDocumentPosition]
//! (#TextDocumentPosition) the request response is of type [DocumentHighlight[]]
//! (#DocumentHighlight) or a Thenable that resolves to such.

use super::{TextDocumentPositionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::DocumentHighlight;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Nullable<DocumentHighlight>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/documentHighlight",
    number_of_params: 1,
    __: None,
};
