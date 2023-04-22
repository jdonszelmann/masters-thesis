use super::Range;

/// Represents a location inside a resource, such as a line
/// inside a text file.
#[derive(Debug, serde::Serialize)]
pub struct Location {
    pub uri: String,

    pub range: Range,
}

/// The Location namespace provides helper functions to work with
/// [Location](#Location) literals.
impl Location {
    /// Creates a Location literal.
    /// @param uri The location's uri.
    /// @param range The location's range.
    pub fn create(uri: String, range: Range) -> Self {
        Location { uri, range }
    }
}
