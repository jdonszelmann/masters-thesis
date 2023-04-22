//! A request to resolve project-wide references for the symbol denoted
//! by the given text document position. The request's parameter is of
//! type [ReferenceParams](#ReferenceParams) the response is of type
//! [Location[]](#Location) or a Thenable that resolves to such.

use super::{TextDocumentPositionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::Location;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Nullable<Vec<Location>>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/references",
    number_of_params: 1,
    __: None,
};
