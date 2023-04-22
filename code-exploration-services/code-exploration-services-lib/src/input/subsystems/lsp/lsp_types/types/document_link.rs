use super::Range;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Value;

/// A document link is a range in a text document that links to an internal or external resource, like another
/// text document or a web site.
#[derive(Debug, serde::Serialize)]
pub struct DocumentLink {
    /// The range this link applies to.
    pub range: Range,

    /// The uri this link points to.
    pub target: Option<String>,

    /// A data entry field that is preserved on a document link between a
    /// DocumentLinkRequest and a DocumentLinkResolveRequest.
    pub data: Option<Value>,
}

/// The DocumentLink namespace provides helper functions to work with
/// [DocumentLink](#DocumentLink) literals.
impl DocumentLink {
    /// Creates a new DocumentLink literal.
    pub fn create(range: Range, target: Option<String>, data: Option<Value>) -> Self {
        DocumentLink {
            range,
            target,
            data,
        }
    }
}
