//! A request sent from the server to the client to modified certain resources.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::RequestType;
use super::{ApplyWorkspaceEditParams, ApplyWorkspaceEditResponse};

pub const TYPE: RequestType<ApplyWorkspaceEditParams, ApplyWorkspaceEditResponse, (), ()> =
    RequestType {
        method: "workspace/applyEdit",
        number_of_params: 1,
        __: None,
    };
