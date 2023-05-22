use super::Position;

/// A range in a text document expressed as (zero-based) start and end positions.
///
/// If you want to specify a range that contains a line including the line ending
/// character(s) then use an end position denoting the start of the next line.
/// For example:
/// ```ts
/// {
///     start: { line: 5, character: 23 }
///     end : { line 6, character : 0 }
/// }
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Range {
    /// The range's start position
    pub start: Position,

    /// The range's end position.
    pub end: Position,
}

/// The Range namespace provides helper functions to work with
/// [Range](#Range) literals.
impl Range {
    /// Create a new Range liternal.
    /// @param start The range's start position.
    /// @param end The range's end position.
    pub fn create_with_position(start: Position, end: Position) -> Self {
        Range { start, end }
    }

    /// Create a new Range liternal.
    /// @param startLine The start line number.
    /// @param startCharacter The start character.
    /// @param endLine The end line number.
    /// @param endCharacter The end character.
    pub fn create_with_number(
        start_line: i32,
        start_character: i32,
        end_line: i32,
        end_character: i32,
    ) -> Self {
        Range {
            start: Position {
                line: start_line,
                character: start_character,
            },
            end: Position {
                line: end_line,
                character: end_character,
            },
        }
    }
}
