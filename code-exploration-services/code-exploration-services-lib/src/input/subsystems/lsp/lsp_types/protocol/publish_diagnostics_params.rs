use crate::input::subsystems::lsp::lsp_types::types::Diagnostic;

/// The publish diagnostic notification's parameters.
#[derive(Debug, serde::Serialize)]
pub struct PublishDiagnosticsParams {
    /// The URI for which diagnostic information is reported.
    pub uri: String,

    /// An array of diagnostic information items.
    pub diagnostics: Vec<Diagnostic>,
}
