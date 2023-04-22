use super::{Color, Range};

/// Represents a color range from a document.
#[derive(Debug, serde::Serialize)]
pub struct ColorInformation {
    /// The range in the document where this color appers.
    pub range: Range,

    /// The actual color value for this color range.
    pub color: Color,
}

/// The ColorInformation namespace provides helper functions to work with
/// [ColorInformation](#ColorInformation) literals.
impl ColorInformation {
    /// Creates a new ColorInformation literal.
    pub fn create(range: Range, color: Color) -> Self {
        ColorInformation {
            range,
            color,
        }
    }
}
