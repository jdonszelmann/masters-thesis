use super::{CodeActionKind, Diagnostic};

/// Contains additional diagnostic information about the context in which
/// a [code action](#CodeActionProvider.provideCodeActions) is run.
#[derive(Debug, serde::Serialize)]
pub struct CodeActionContext {
    /// An array of diagnostics.
    pub diagnostics: Vec<Diagnostic>,

    /// Requested kind of actions to return.
    ///
    /// Actions not of this kind are filtered out by the client before being shown. So servers
    /// can omit computing them.
    pub only: Option<Vec<CodeActionKind>>,
}

/// The CodeActionContext namespace provides helper functions to work with
/// [CodeActionContext](#CodeActionContext) literals.
impl CodeActionContext {
    /// Creates a new CodeActionContext literal.
    pub fn create(diagnostics: Vec<Diagnostic>, only: Option<Vec<CodeActionKind>>) -> Self {
        CodeActionContext { diagnostics, only }
    }
}
