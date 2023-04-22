//! The initialize request is sent from the client to the server.
//! It is sent once as the request after starting up the server.
//! The requests parameter is of type [InitializeParams](#InitializeParams)
//! the response if of type [InitializeResult](#InitializeResult) of a Thenable that
//! resolves to such.

use super::{InitializeError, InitializeParams, InitializeResult};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;

pub const TYPE: RequestType<InitializeParams, InitializeResult, InitializeError, ()> =
    RequestType {
        method: "initialize",
        number_of_params: 1,
        __: None,
    };
