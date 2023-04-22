//! A request to rename a symbol.

use super::{RenameParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::WorkspaceEdit;

pub const TYPE: RequestType<
    RenameParams,
    Nullable<WorkspaceEdit>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/rename",
    number_of_params: 1,
    __: None,
};
