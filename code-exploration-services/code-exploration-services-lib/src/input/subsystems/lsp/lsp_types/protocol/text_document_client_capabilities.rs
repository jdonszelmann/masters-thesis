use super::{
    CodeActionCapabilities, CodeLensCapabilities, ColorProviderCapabilities,
    CompletionCapabilities, DefinitionCapabilities, DocumentHighlightCapabilities,
    DocumentLinkCapabilities, DocumentSymbolCapabilities, FormattingCapabilities,
    HoverCapabilities, ImplementationCapabilities, OnTypeFormattingCapabilities,
    PublishDiagnosticsCapabilities, RangeFormattingCapabilities, ReferencesCapabilities,
    RenameCapabilities, SignatureHelpCapabilities, SynchronizationCapabilities,
    TypeDefinitionCapabilities,
};

/// Text document specific client capabilities.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentClientCapabilities {
    /// Defines which synchronization capabilities the client supports.
    pub synchronization: Option<SynchronizationCapabilities>,

    /// Capabilities specific to the `textDocument/completion`
    pub completion: Option<CompletionCapabilities>,

    /// Capabilities specific to the `textDocument/hover`
    pub hover: Option<HoverCapabilities>,

    /// Capabilities specific to the `textDocument/signatureHelp`
    pub signature_help: Option<SignatureHelpCapabilities>,

    /// Capabilities specific to the `textDocument/references`
    pub references: Option<ReferencesCapabilities>,

    /// Capabilities specific to the `textDocument/documentHighlight`
    pub document_highlight: Option<DocumentHighlightCapabilities>,

    /// Capabilities specific to the `textDocument/documentSymbol`
    pub document_symbol: Option<DocumentSymbolCapabilities>,

    /// Capabilities specific to the `textDocument/formatting`
    pub formatting: Option<FormattingCapabilities>,

    /// Capabilities specific to the `textDocument/rangeFormatting`
    pub range_formatting: Option<RangeFormattingCapabilities>,

    /// Capabilities specific to the `textDocument/onTypeFormatting`
    pub on_type_formatting: Option<OnTypeFormattingCapabilities>,

    /// Capabilities specific to the `textDocument/definition`
    pub definition: Option<DefinitionCapabilities>,

    /// Capabilities specific to the `textDocument/typeDefinition`
    ///
    /// Since 3.6.0
    pub type_definition: Option<TypeDefinitionCapabilities>,

    /// Capabilities specific to the `textDocument/implementation`.
    ///
    /// Since 3.6.0
    pub implementation: Option<ImplementationCapabilities>,

    /// Capabilities specific to the `textDocument/codeAction`
    pub code_action: Option<CodeActionCapabilities>,

    /// Capabilities specific to the `textDocument/codeLens`
    pub code_lens: Option<CodeLensCapabilities>,

    /// Capabilities specific to the `textDocument/documentLink`
    pub document_link: Option<DocumentLinkCapabilities>,

    /// Capabilities specific to the `textDocument/documentColor` and the
    /// `textDocument/colorPresentation` request.
    ///
    /// Since 3.6.0
    pub color_provider: Option<ColorProviderCapabilities>,

    /// Capabilities specific to the `textDocument/rename`
    pub rename: Option<RenameCapabilities>,

    /// Capabilities specific to `textDocument/publishDiagnostics`.
    pub publish_diagnostics: Option<PublishDiagnosticsCapabilities>,
}
