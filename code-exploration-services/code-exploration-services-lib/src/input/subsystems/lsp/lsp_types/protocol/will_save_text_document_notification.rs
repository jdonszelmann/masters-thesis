//! A document will save request is sent from the client to the server before
//! the document is actually saved. The request can return an array of TextEdits
//! which will be applied to the text document before it is saved. Please note that
//! clients might drop results if computing the text edits took too long or if a
//! server constantly fails on this request. This is done to keep the save fast and
//! reliable.

use super::{WillSaveTextDocumentParams, TextDocumentRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{RequestType, Nullable};
use crate::input::subsystems::lsp::lsp_types::types::TextEdit;

pub const TYPE: RequestType<WillSaveTextDocumentParams, Nullable<Vec<TextEdit>>, (), TextDocumentRegistrationOptions> =
    RequestType {
        method: "textDocument/willSaveWaitUntil",
        number_of_params: 1,
        __: None,
    };
