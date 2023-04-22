use super::{ClientCapabilities, InitialTraceSetting, WorkspaceFolder};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, Value};

/// The initialize parameters
#[derive(Debug, serde::Serialize)]
pub struct InitializeParams {
    /// The process Id of the parent process that started
    /// the server.
    pub process_id: i32,

    /// The rootPath of the workspace. Is null
    /// if no folder is open.
    ///
    /// @deprecated in favour of rootUri.
    pub root_path: Option<Nullable<String>>,

    /// The rootUri of the workspace. Is null if no
    /// folder is open. If both `rootPath` and `rootUri` are set
    /// `rootUri` wins.
    pub root_uri: Nullable<String>,

    /// The capabilities provided by the client (editor or tool)
    pub capabilities: ClientCapabilities,

    /// User provided initialization options.
    pub initialization_options: Option<Value>,

    /// The initial trace setting. If omitted trace is disabled ('off').
    pub trace: Option<InitialTraceSetting>,

    /// The workspace folders configured in the client when the server starts.
    /// This property is only available if the client supports workspace folders.
    /// It can be `null` if the client supports workspace folders but none are
    /// configured.
    ///
    /// Since 3.6.0
    pub workspace_folders: Option<Nullable<Vec<WorkspaceFolder>>>,
}
