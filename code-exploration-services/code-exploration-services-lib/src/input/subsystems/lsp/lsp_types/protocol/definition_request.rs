//! A request to resolve the definition location of a symbol at a given text
//! document position. The request's parameter is of type [TextDocumentPosition]
//! (#TextDocumentPosition) the response is of type [Definition](#Definition) or a
//! Thenable that resolves to such.

use super::{TextDocumentPositionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::Definition;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Nullable<Definition>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/definition",
    number_of_params: 1,
    __: None,
};
