//! A request to list project-wide symbols matching the query string given
//! by the [WorkspaceSymbolParams](#WorkspaceSymbolParams). The response is
//! of type [SymbolInformation[]](#SymbolInformation) or a Thenable that
//! resolves to such.

use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, RequestType};
use crate::input::subsystems::lsp::lsp_types::types::{SymbolInformation, WorkspaceSymbolParams};

pub const TYPE: RequestType<WorkspaceSymbolParams, Nullable<Vec<SymbolInformation>>, (), ()> =
    RequestType {
        method: "workspace/symbol",
        number_of_params: 1,
        __: None,
    };
