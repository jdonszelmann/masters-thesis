//! A request to resolve the implementation locations of a symbol at a given text
//! document position. The request's parameter is of type [TextDocumentPositioParams]
//! (#TextDocumentPositionParams) the response is of type [Definition](#Definition) or a
//! Thenable that resolves to such.

use super::{TextDocumentPositionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::Definition;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Vec<Definition>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/implementation",
    number_of_params: 1,
    __: None,
};
