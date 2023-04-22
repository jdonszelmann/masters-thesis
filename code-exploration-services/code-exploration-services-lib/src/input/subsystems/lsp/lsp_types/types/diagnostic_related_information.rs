use super::Location;

/// Represents a related message and source code location for a diagnostic. This should be
/// used to point to code locations that cause or related to a diagnostics, e.g when duplicating
/// a symbol in a scope.
#[derive(Debug, serde::Serialize)]
pub struct DiagnosticRelatedInformation {
    /// The location of this related diagnostic information.
    pub location: Location,

    /// The message of this related diagnostic information.
    pub message: String,
}

/// The DiagnosticRelatedInformation namespace provides helper functions to work with
/// [DiagnosticRelatedInformation](#DiagnosticRelatedInformation) literals.
impl DiagnosticRelatedInformation {
    /// Creates a new DiagnosticRelatedInformation literal.
    pub fn create(location: Location, message: String) -> Self {
        DiagnosticRelatedInformation {
            location,
            message
        }
    }
}
