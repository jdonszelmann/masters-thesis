use crate::input::subsystems::lsp::lsp_types::types::MarkupKind;

/// The client supports the following `CompletionItem` specific
/// capabilities.
#[derive(Debug, serde::Serialize)]
pub struct CompletionItemCapabilities {
    /// Client supports snippets as insert text.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    pub snippet_support: Option<bool>,

    /// Client supports commit characters on a completion item.
    pub commit_characters_support: Option<bool>,

    /// Client supports the follow content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,

    /// Client supports the deprecated property on a completion item.
    pub deprecated_support: Option<bool>,

    /// Client supports the preselect property on a completion item.
    pub preselect_support: Option<bool>,
}
