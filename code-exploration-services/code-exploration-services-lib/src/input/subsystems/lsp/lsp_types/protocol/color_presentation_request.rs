//! A request to list all presentation for a color. The request's
//! parameter is of type [ColorPresentationParams](#ColorPresentationParams) the
//! response is of type [ColorInformation[]](#ColorInformation) or a Thenable
//! that resolves to such.

use super::{ColorPresentationParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::ColorPresentation;

pub const TYPE: RequestType<
    ColorPresentationParams,
    Vec<ColorPresentation>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/colorPresentation",
    number_of_params: 1,
    __: None,
};
