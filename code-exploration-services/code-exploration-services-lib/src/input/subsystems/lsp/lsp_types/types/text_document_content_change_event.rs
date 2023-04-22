use super::Range;

/// An event describing a change to a text document. If range and rangeLength are omitted
/// the new text is considered to be the full content of the document.
#[derive(Debug, serde::Serialize)]
pub struct TextDocumentContentChangeEvent {
    /// The range of the document that changed.
    pub range: Option<Range>,

    /// The length of the range that got replaced.
    pub range_length: Option<i32>,

    /// The new text of the document.
    pub text: String,
}
