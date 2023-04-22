use super::{SignatureHelpRegistrationOptions, TextDocumentPositionParams};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::SignatureHelp;

pub const TYPE: RequestType<
    TextDocumentPositionParams,
    Nullable<SignatureHelp>,
    (),
    SignatureHelpRegistrationOptions,
> = RequestType {
    method: "textDocument/signatureHelp",
    number_of_params: 1,
    __: None,
};
