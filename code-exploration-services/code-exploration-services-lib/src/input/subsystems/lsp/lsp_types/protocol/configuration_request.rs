//! The 'workspace/configuration' request is sent from the server to the client to fetch a certain
//! configuration setting.

use super::ConfigurationParams;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{RequestType, Value};

pub const TYPE: RequestType<ConfigurationParams, Vec<Value>, (), ()> = RequestType {
    method: "workspace/configuration",
    number_of_params: 1,
    __: None,
};
