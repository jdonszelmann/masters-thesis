//! A shutdown request is sent from the client to the server.
//! It is sent once when the client decides to shutdown the
//! server. The only notification that is sent after a shutdown request
//! is the exit event.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType0;

pub const TYPE: RequestType0<(), (), ()> = RequestType0 {
    method: "shutdown",
    number_of_params: 0,
    __: None,
};
