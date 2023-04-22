use super::{CompletionItemCapabilities, CompletionItemKindCapabilities};

/// Capabilities specific to the `textDocument/completion`
#[derive(Debug, serde::Serialize)]
pub struct CompletionCapabilities {
    /// Whether completion supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports the following `CompletionItem` specific
    /// capabilities.
    pub completion_item: Option<CompletionItemCapabilities>,

    pub completion_item_kind: Option<CompletionItemKindCapabilities>,

    /// The client supports to send additional context information for a
    /// `textDocument/completion` requestion.
    pub context_support: Option<bool>,
}
