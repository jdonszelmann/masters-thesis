//! A request to provide commands for the given text document and range.

use super::{CodeActionParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType, Union};
use crate::input::subsystems::lsp::lsp_types::types::{CodeAction, Command};

pub const TYPE: RequestType<
    CodeActionParams,
    Nullable<Vec<Union<Command, CodeAction>>>,
    (),
    TextDocumentRegistrationOptions,
> = RequestType {
    method: "textDocument/codeAction",
    number_of_params: 1,
    __: None,
};
