//! Request to resolve additional information for a given completion item.The request's
//! parameter is of type [CompletionItem](#CompletionItem) the response
//! is of type [CompletionItem](#CompletionItem) or a Thenable that resolves to such.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use crate::input::subsystems::lsp::lsp_types::types::CompletionItem;

pub const TYPE: RequestType<CompletionItem, CompletionItem, (), ()> = RequestType {
    method: "completionItem/resolve",
    number_of_params: 1,
    __: None,
};
