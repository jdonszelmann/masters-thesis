use super::{DocumentHighlightKind, Range};

/// A document highlight is a range inside a text document which deserves
/// special attention. Usually a document highlight is visualized by changing
/// the background color of its range.
#[derive(Debug, serde::Serialize)]
pub struct DocumentHighlight {
    /// The range this highlight applies to.
    pub range: Range,

    /// The highlight kind, default is [text](#DocumentHighlightKind.Text).
    pub kind: Option<DocumentHighlightKind>,
}

/// DocumentHighlight namespace to provide helper functions to work with
/// [DocumentHighlight](#DocumentHighlight) literals.
impl DocumentHighlight {
    /// Create a DocumentHighlight object.
    /// @param range The range the highlight applies to.
    pub fn create(range: Range, kind: Option<DocumentHighlightKind>) -> Self {
        DocumentHighlight { range, kind }
    }
}
