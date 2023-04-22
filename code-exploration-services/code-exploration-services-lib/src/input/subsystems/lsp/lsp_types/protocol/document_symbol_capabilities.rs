use super::SymbolKindCapabilities;

/// Capabilities specific to the `textDocument/documentSymbol`
#[derive(Debug, serde::Serialize)]
pub struct DocumentSymbolCapabilities {
    /// Whether document symbol supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Specific capabilities for the `SymbolKind`.
    pub symbol_kind: Option<SymbolKindCapabilities>,

    /// The client support hierarchical document symbols.
    pub hierarchical_document_symbol_support: Option<bool>,
}
