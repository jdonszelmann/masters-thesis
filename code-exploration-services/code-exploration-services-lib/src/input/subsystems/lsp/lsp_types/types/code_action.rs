use super::{CodeActionKind, Command, Diagnostic, WorkspaceEdit};

/// A code action represents a change that can be performed in code, e.g. to fix a problem or
/// to refactor code.
///
/// A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is applied first, then the `command` is executed.
#[derive(Debug, serde::Serialize)]
pub struct CodeAction {
    /// A short, human-readable, title for this code action.
    pub title: String,

    /// The kind of the code action.
    ///
    /// Used to filter code actions.
    pub kind: Option<CodeActionKind>,

    /// The diagnostics that this code action resolves.
    pub diagnostics: Option<Vec<Diagnostic>>,

    /// The workspace edit this code action performs.
    pub edit: Option<WorkspaceEdit>,

    /// A command this code action executes. If a code action
    /// provides a edit and a command, first the edit is
    /// executed and then the command.
    pub command: Option<Command>,
}

impl CodeAction {
    /// Creates a new code action.
    ///
    /// @param title The title of the code action.
    /// @param command The command to execute.
    /// @param kind The kind of the code action.
    pub fn create_with_command(
        title: String,
        command: Command,
        kind: Option<CodeActionKind>,
    ) -> Self {
        CodeAction {
            title,
            kind,
            diagnostics: None,
            edit: None,
            command: Some(command),
        }
    }

    /// Creates a new code action.
    ///
    /// @param title The title of the code action.
    /// @param command The command to execute.
    /// @param kind The kind of the code action.
    pub fn create_with_workspace_edit(
        title: String,
        edit: WorkspaceEdit,
        kind: Option<CodeActionKind>,
    ) -> Self {
        CodeAction {
            title,
            kind,
            diagnostics: None,
            edit: Some(edit),
            command: None,
        }
    }
}
