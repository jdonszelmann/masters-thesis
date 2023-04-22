use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// Represents a reference to a command. Provides a title which
/// will be used to represent a command in the UI and, optionally,
/// an array of arguments which will be passed to the command handler
/// function when invoked.
#[derive(Debug, serde::Serialize)]
pub struct Command {
    /// Title of the command, like `save`.
    pub title: String,

    /// The identifier of the actual command handler.
    pub command: String,

    /// Arguments that the command handler should be
    /// invoked with.
    pub arguments: Option<Vec<Value>>,
}

/// The Command namespace provides helper functions to work with
/// [Command](#Command) literals.
impl Command {
    /// Creates a new Command literal.
    pub fn create(title: String, command: String, arguments: Option<Vec<Value>>) -> Self {
        Command {
            title,
            command,
            arguments,
        }
    }
}
