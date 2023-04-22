//! A request to to format a whole document.

use super::{DocumentFormattingParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::TextEdit;

pub const TYPE: RequestType<
    DocumentFormattingParams,
    Nullable<Vec<TextEdit>>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/formatting",
    number_of_params: 1,
    __: None,
};
