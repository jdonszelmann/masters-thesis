//! Request to resolve additional information for a given document link. The request's
//! parameter is of type [DocumentLink](#DocumentLink) the response
//! is of type [DocumentLink](#DocumentLink) or a Thenable that resolves to such.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::DocumentLink;

pub const TYPE: RequestType<DocumentLink, DocumentLink, (), ()> = RequestType {
    method: "documentLink/resolve",
    number_of_params: 1,
    __: None,
};
