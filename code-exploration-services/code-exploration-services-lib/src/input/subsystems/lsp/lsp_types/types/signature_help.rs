use super::SignatureInformation;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;

/// Signature help represents the signature of something
/// callable. There can be multiple signature but only one
/// active and only one active parameter.
#[derive(Debug, serde::Serialize)]
pub struct SignatureHelp {
    /// One or more signatures.
    pub signatures: Vec<SignatureInformation>,

    /// The active signature. Set to `null` if no
    /// signatures exist.
    pub active_signature: Nullable<i32>,

    /// The active parameter of the active signature. Set to `null`
    /// if the active signature has no parameters.
    pub active_parameter: Nullable<i32>,
}
