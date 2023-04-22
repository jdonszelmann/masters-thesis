//! A request send from the client to the server to execute a command. The request might return
//! a workspace edit which the client will apply to the workspace.

use super::{ExecuteCommandParams, ExecuteCommandRegistrationOptions};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType, Value};

pub const TYPE: RequestType<
    ExecuteCommandParams,
    Nullable<Value>,
    (),
    ExecuteCommandRegistrationOptions,
> = RequestType {
    method: "workspace/executeCommand",
    number_of_params: 1,
    __: None,
};
