//! The `workspace/workspaceFolders` is sent from the server to the client to fetch the open workspace folders.

use super::WorkspaceFolder;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType0};

pub const TYPE: RequestType0<Nullable<Vec<WorkspaceFolder>>, (), ()> = RequestType0 {
    method: "workspace/workspaceFolders",
    number_of_params: 0,
    __: None,
};
