//! A request to provide code lens for the given text document.

use super::{CodeLensParams, CodeLensRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::CodeLens;

pub const TYPE: RequestType<
    CodeLensParams,
    Nullable<Vec<CodeLens>>,
    (),
    CodeLensRegistrationOptions,
> = RequestType {
    method: "textDocument/codeLens",
    number_of_params: 1,
    __: None,
};
