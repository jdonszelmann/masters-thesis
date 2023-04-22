use super::SymbolKindCapabilities;

/// Capabilities specific to the `workspace/symbol` request.
#[derive(Debug, serde::Serialize)]
pub struct SymbolCapabilities {
    /// Symbol request supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
    pub symbol_kind: Option<SymbolKindCapabilities>,
}
