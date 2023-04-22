use super::{Position, Range};

/// A text edit applicable to a text document.
#[derive(Debug, serde::Serialize)]
pub struct TextEdit {
    /// The range of the text document to be manipulated. To insert
    /// text into a document create a range where start === end.
    pub range: Range,

    /// The string to be inserted. For delete operations use an
    /// empty string.
    pub new_text: String,
}

/// The TextEdit namespace provides helper function to create replace,
/// insert and delete edits more easily.
impl TextEdit {
    /// Creates a replace text edit.
    /// @param range The range of text to be replaced.
    /// @param newText The new text.
    pub fn replace(range: Range, new_text: String) -> Self {
        TextEdit { range, new_text }
    }

    /// Creates a insert text edit.
    /// @param position The position to insert the text at.
    /// @param newText The text to be inserted.
    pub fn insert(position: Position, new_text: String) -> Self {
        TextEdit { range: Range { start: position, end: position }, new_text }
    }
    /// Creates a delete text edit.
    /// @param range The range of text to be deleted.
    pub fn delete(range: Range) -> Self {
        TextEdit { range, new_text: "".to_string() }
    }
}
