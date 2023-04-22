use super::{
    CodeLensOptions, ColorProviderOptions, CompletionOptions, DocumentLinkOptions,
    DocumentOnTypeFormattingOptions, ExecuteCommandOptions, SignatureHelpOptions,
    StaticRegistrationOptions, TextDocumentRegistrationOptions, TextDocumentSyncKind,
    TextDocumentSyncOptions, WorkspaceServerCapabilities,
};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Intersection, Intersection3, Union, Union3, Value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerCapabilities {
    /// Defines how text documents are synced. Is either a detailed structure defining each notification or
    /// for backwards compatibility the TextDocumentSyncKind number. If omitted it defaults to `TextDocumentSyncKind.None`.
    pub text_document_sync: Option<Union<TextDocumentSyncOptions, TextDocumentSyncKind>>,

    /// The server provides hover support.
    pub hover_provider: Option<bool>,

    /// The server provides completion support.
    pub completion_provider: Option<CompletionOptions>,

    /// The server provides signature help support.
    pub signature_help_provider: Option<SignatureHelpOptions>,

    /// The server provides goto definition support.
    pub definition_provider: Option<bool>,

    /// The server provides Goto Type Definition support.
    ///
    /// Since 3.6.0
    pub type_definition_provider: Option<
        Union<bool, Intersection<TextDocumentRegistrationOptions, StaticRegistrationOptions>>,
    >,

    /// The server provides Goto Implementation support.
    ///
    /// Since 3.6.0
    pub implementation_provider: Option<
        Union<bool, Intersection<TextDocumentRegistrationOptions, StaticRegistrationOptions>>,
    >,

    /// The server provides find references support.
    pub references_provider: Option<bool>,

    /// The server provides document highlight support.
    pub document_highlight_provider: Option<bool>,

    /// The server provides document symbol support.
    pub document_symbol_provider: Option<bool>,

    /// The server provides workspace symbol support.
    pub workspace_symbol_provider: Option<bool>,

    /// The server provides code actions.
    pub code_action_provider: Option<bool>,

    /// The server provides code lens.
    pub code_lens_provider: Option<CodeLensOptions>,

    /// The server provides document formatting.
    pub document_formatting_provider: Option<bool>,

    /// The server provides document range formatting.
    pub document_range_formatting_provider: Option<bool>,

    /// The server provides document formatting on typing.
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,

    /// The server provides rename support.
    pub rename_provider: Option<bool>,

    /// The server provides document link support.
    pub document_link_provider: Option<DocumentLinkOptions>,

    /// The server provides color provider support.
    ///
    /// Since 3.6.0
    pub color_provider: Option<
        Union3<
            bool,
            ColorProviderOptions,
            Intersection3<
                ColorProviderOptions,
                TextDocumentRegistrationOptions,
                StaticRegistrationOptions,
            >,
        >,
    >,

    /// The server provides execute command support.
    pub execute_command_provider: Option<ExecuteCommandOptions>,

    /// Workspace specific server capabilities
    pub workspace: Option<WorkspaceServerCapabilities>,

    /// Experimental server capabilities.
    pub experimental: Option<Value>,
}
