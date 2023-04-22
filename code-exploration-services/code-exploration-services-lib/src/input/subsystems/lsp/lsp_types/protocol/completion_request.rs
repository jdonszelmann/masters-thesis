//! Request to request completion at a given text document position. The request's
//! parameter is of type [TextDocumentPosition](#TextDocumentPosition) the response
//! is of type [CompletionItem[]](#CompletionItem) or [CompletionList](#CompletionList)
//! or a Thenable that resolves to such.
//!
//! The request can delay the computation of the [`detail`](#CompletionItem.detail)
//! and [`documentation`](#CompletionItem.documentation) properties to the `completionItem/resolve`
//! request. However, properties that are needed for the initial sorting and filtering, like `sortText`,
//! `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.

use super::{CompletionParams, CompletionRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType, Union};
use crate::input::subsystems::lsp::lsp_types::types::{CompletionItem, CompletionList};

pub const TYPE: RequestType<
    CompletionParams,
    Nullable<Union<Vec<CompletionItem>, CompletionList>>,
    (),
    CompletionRegistrationOptions,
> = RequestType {
    method: "textDocument/completion",
    number_of_params: 1,
    __: None,
};
