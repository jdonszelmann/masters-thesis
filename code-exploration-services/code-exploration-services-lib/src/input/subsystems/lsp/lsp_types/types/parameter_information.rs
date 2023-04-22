use super::MarkupContent;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

/// Represents a parameter of a callable-signature. A parameter can
/// have a label and a doc-comment.
#[derive(Debug, serde::Serialize)]
pub struct ParameterInformation {
    /// The label of this signature. Will be shown in
    /// the UI.
    pub label: String,

    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<Union<String, MarkupContent>>,
}

/// The ParameterInformation namespace provides helper functions to work with
/// [ParameterInformation](#ParameterInformation) literals.
impl ParameterInformation {
    /// Creates a new parameter information literal.
    ///
    /// @param label A label string.
    /// @param documentation A doc string.
    pub fn create(label: String, documentation: Option<String>) -> Self {
        match documentation {
            Some(documentation) => ParameterInformation {
                label,
                documentation: Some(Union::for0(documentation)),
            },
            _ => ParameterInformation {
                label,
                documentation: None,
            },
        }
    }
}
