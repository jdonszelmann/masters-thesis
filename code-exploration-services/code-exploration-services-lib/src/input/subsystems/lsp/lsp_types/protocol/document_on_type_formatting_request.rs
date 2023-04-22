//! A request to format a document on type.

use super::{DocumentOnTypeFormattingParams, DocumentOnTypeFormattingRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::TextEdit;

pub const TYPE: RequestType<
    DocumentOnTypeFormattingParams,
    Nullable<Vec<TextEdit>>,
    (),
    DocumentOnTypeFormattingRegistrationOptions,
> = RequestType {
    method: "textDocument/onTypeFormatting",
    number_of_params: 1,
    __: None,
};
