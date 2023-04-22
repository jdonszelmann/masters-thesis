//! A request to provide document links

use super::{DocumentLinkParams, DocumentLinkRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::DocumentLink;

pub const TYPE: RequestType<
    DocumentLinkParams,
    Nullable<Vec<DocumentLink>>,
    (),
    DocumentLinkRegistrationOptions,
> = RequestType {
    method: "textDocument/documentLink",
    number_of_params: 1,
    __: None,
};
