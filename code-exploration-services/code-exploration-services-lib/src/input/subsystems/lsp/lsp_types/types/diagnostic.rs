use super::{DiagnosticRelatedInformation, DiagnosticSeverity, Range};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

/// Represents a diagnostic, such as a compiler error or warning. Diagnostic objects
/// are only valid in the scope of a resource.
#[derive(Debug, serde::Serialize)]
pub struct Diagnostic {
    /// The range at which the message applies
    pub range: Range,

    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    pub severity: Option<DiagnosticSeverity>,

    /// The diagnostic's code, which might appear in the user interface.
    pub code: Option<Union<i32, String>>,

    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    pub source: Option<String>,

    /// The diagnostic's message.
    pub message: String,

    /// An array of related diagnostic information, e.g. when symbol-names within
    /// a scope collide all definitions can be marked via this property.
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

/// The Diagnostic namespace provides helper functions to work with
/// [Diagnostic](#Diagnostic) literals.
impl Diagnostic {
    /// Creates a new Diagnostic literal.
    pub fn create(
        range: Range,
        message: String,
        severity: Option<DiagnosticSeverity>,
        code: Option<Union<i32, String>>,
        source: Option<String>,
        related_information: Option<Vec<DiagnosticRelatedInformation>>,
    ) -> Self {
        Diagnostic {
            range,
            message,
            severity,
            code,
            source,
            related_information,
        }
    }
}
