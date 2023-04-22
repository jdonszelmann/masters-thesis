//! A request to list all symbols found in a given text document. The request's
//! parameter is of type [TextDocumentIdentifier](#TextDocumentIdentifier) the
//! response is of type [SymbolInformation[]](#SymbolInformation) or a Thenable
//! that resolves to such.

use super::TextDocumentRegistrationOptions;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType, Union};
use crate::input::subsystems::lsp::lsp_types::types::{DocumentSymbol, DocumentSymbolParams, SymbolInformation};

pub const TYPE: RequestType<
    DocumentSymbolParams,
    Nullable<Union<SymbolInformation, DocumentSymbol>>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/documentSymbol",
    number_of_params: 1,
    __: None,
};
