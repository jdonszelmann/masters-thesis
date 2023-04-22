//! The `client/registerCapability` request is sent from the server to the client to register a new capability
//! handler on the client side.

use super::RegistrationParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;

pub const TYPE: RequestType<RegistrationParams, (), (), ()> = RequestType {
    method: "client/registerCapability",
    number_of_params: 1,
    __: None,
};
