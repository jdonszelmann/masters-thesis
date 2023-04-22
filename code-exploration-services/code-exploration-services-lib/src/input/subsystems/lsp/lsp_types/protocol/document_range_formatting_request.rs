//! A request to to format a range in a document.

use super::{DocumentRangeFormattingParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::TextEdit;

pub const TYPE: RequestType<
    DocumentRangeFormattingParams,
    Nullable<Vec<TextEdit>>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/rangeFormatting",
    number_of_params: 1,
    __: None,
};
