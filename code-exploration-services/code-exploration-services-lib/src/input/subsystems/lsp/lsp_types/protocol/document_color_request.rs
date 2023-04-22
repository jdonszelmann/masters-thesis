//! A request to list all color symbols found in a given text document. The request's
//! parameter is of type [DocumentColorParams](#DocumentColorParams) the
//! response is of type [ColorInformation[]](#ColorInformation) or a Thenable
//! that resolves to such.

use super::{DocumentColorParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::ColorInformation;

pub const TYPE: RequestType<
    DocumentColorParams,
    Vec<ColorInformation>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/documentColor",
    number_of_params: 1,
    __: None,
};
