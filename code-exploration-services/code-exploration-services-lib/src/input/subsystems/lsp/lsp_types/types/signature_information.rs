use super::{MarkupContent, ParameterInformation};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

/// Represents the signature of something callable. A signature
/// can have a label, like a function-name, a doc-comment, and
/// a set of parameters.
#[derive(Debug, serde::Serialize)]
pub struct SignatureInformation {
    /// The label of this signature. Will be shown in
    /// the UI.
    pub label: String,

    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<Union<String, MarkupContent>>,

    /// The parameters of this signature.
    pub parameters: Option<Vec<ParameterInformation>>,
}

/// The SignatureInformation namespace provides helper functions to work with
/// [SignatureInformation](#SignatureInformation) literals.
impl SignatureInformation {
    pub fn create(
        label: String,
        documentation: Option<String>,
        parameters: Option<Vec<ParameterInformation>>,
    ) -> Self {
        SignatureInformation {
            label,
            documentation: match documentation {
                Some(documentation) => Some(Union::for0(documentation)),
                _ => None,
            },
            parameters: match parameters {
                Some(parameters) => Some(parameters),
                _ => Some(Vec::new()),
            },
        }
    }
}
