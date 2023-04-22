//! The `client/unregisterCapability` request is sent from the server to the client to unregister a previously registered capability
//! handler on the client side.

use super::UnregistrationParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;

pub const TYPE: RequestType<UnregistrationParams, (), (), ()> = RequestType {
    method: "client/unregisterCapability",
    number_of_params: 1,
    __: None,
};
