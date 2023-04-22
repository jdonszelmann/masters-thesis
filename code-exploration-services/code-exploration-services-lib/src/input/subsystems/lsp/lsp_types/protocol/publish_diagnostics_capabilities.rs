/// Capabilities specific to `textDocument/publishDiagnostics`.
#[derive(Debug, serde::Serialize)]
pub struct PublishDiagnosticsCapabilities {
    /// Whether the clients accepts diagnostics with related information.
    pub dynamic_registration: Option<bool>,
}
