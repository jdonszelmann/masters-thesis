/// Position in a text document expressed as zero-based line and character offset.
/// The offsets are based on a UTF-16 string representation. So a string of the form
/// `a𐐀b` the character offset of the character `a` is 0, the character offset of `𐐀`
/// is 1 and the character offset of b is 3 since `𐐀` is represented using two code
/// units in UTF-16.
///
/// Positions are line end character agnostic. So you can not specify a position that
/// denotes `\r|\n` or `\n|` where `|` represents the character offset.
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Position {
    /// Line position in a document (zero-based).
    /// If a line number is greater than the number of lines in a document, it defaults back to the number of lines in the document.
    /// If a line number is negative, it defaults to 0.
    pub line: i32,

    /// Character offset on a line in a document (zero-based). Assuming that the line is
    /// represented as a string, the `character` value represents the gap between the
    /// `character` and `character + 1`.
    ///
    /// If the character value is greater than the line length it defaults back to the
    /// line length.
    /// If a line number is negative, it defaults to 0.
    pub character: i32,
}

/// The Position namespace provides helper functions to work with
/// [Position](#Position) literals.
impl Position {
    /// Creates a new Position literal from the given line and character.
    /// @param line The position's line.
    /// @param character The position's character.
    pub fn create(line: i32, character: i32) -> Self {
        Position { line, character }
    }
}
