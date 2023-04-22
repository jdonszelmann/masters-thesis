use crate::input::subsystems::lsp::lsp_types::types::SymbolKind;

/// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
#[derive(Debug, serde::Serialize)]
pub struct SymbolKindCapabilities {
    /// The symbol kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    ///
    /// If this property is not present the client only supports
    /// the symbol kinds from `File` to `Array` as defined in
    /// the initial version of the protocol.
    pub value_set: Option<Vec<SymbolKind>>,
}
