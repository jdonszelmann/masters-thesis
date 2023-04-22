use super::{Command, Range};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// A code lens represents a [command](#Command) that should be shown along with
/// source text, like the number of references, a way to run tests, etc.
///
/// A code lens is _unresolved_ when no command is associated to it. For performance
/// reasons the creation of a code lens and resolving should be done to two stages.
#[derive(Debug, serde::Serialize)]
pub struct CodeLens {
    /// The range in which this code lens is valid. Should only span a single line.
    pub range: Range,

    /// The command this code lens represents.
    pub command: Option<Command>,

    /// An data entry field that is preserved on a code lens item between
    /// a [CodeLensRequest](#CodeLensRequest) and a [CodeLensResolveRequest]
    /// (#CodeLensResolveRequest)
    pub data: Option<Value>,
}

/// The CodeLens namespace provides helper functions to work with
/// [CodeLens](#CodeLens) literals.
impl CodeLens {
    /// Creates a new CodeLens literal.
    pub fn create(range: Range, data: Option<Value>) -> Self {
        CodeLens {
            range,
            command: None,
            data,
        }
    }
}
