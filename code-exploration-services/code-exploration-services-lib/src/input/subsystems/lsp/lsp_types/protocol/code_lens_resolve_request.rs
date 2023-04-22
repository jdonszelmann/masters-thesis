//! A request to resolve a command for a given code lens.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::CodeLens;

pub const TYPE: RequestType<CodeLens, CodeLens, (), ()> = RequestType {
    method: "codeLens/resolve",
    number_of_params: 1,
    __: None,
};
