//! Request to request hover information at a given text document position. The request's
//! parameter is of type [TextDocumentPosition](#TextDocumentPosition) the response is of
//! type [Hover](#Hover) or a Thenable that resolves to such.

use super::{TextDocumentPositionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::Hover;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Nullable<Hover>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/hover",
    number_of_params: 1,
    __: None,
};
