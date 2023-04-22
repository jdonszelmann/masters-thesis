use super::{Location, Range, SymbolKind};

/// Represents information about programming constructs like variables, classes,
/// interfaces etc.
#[derive(Debug, serde::Serialize)]
pub struct SymbolInformation {
    /// The name of this symbol.
    pub name: String,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    /// Indicates if this symbol is deprecated.
    pub deprecated: Option<bool>,

    /// The location of this symbol. The location's range is used by a tool
    /// to reveal the location in the editor. If the symbol is selected in the
    /// tool the range's start information is used to position the cursor. So
    /// the range usually spans more than the actual symbol's name and does
    /// normally include thinks like visibility modifiers.
    ///
    /// The range doesn't have to denote a node range in the sense of a abstract
    /// syntax tree. It can therefore not be used to re-construct a hierarchy of
    /// the symbols.
    pub location: Location,

    /// The name of the symbol containing this symbol. This information is for
    /// user interface purposes (e.g. to render a qualifier in the user interface
    /// if necessary). It can't be used to re-infer a hierarchy for the document
    /// symbols.
    pub container_name: Option<String>,
}

impl SymbolInformation {
    /// Creates a new symbol information literal.
    ///
    /// @param name The name of the symbol.
    /// @param kind The kind of the symbol.
    /// @param range The range of the location of the symbol.
    /// @param uri The resource of the location of symbol, defaults to the current document.
    /// @param containerName The name of the symbol containing the symbol.
    pub fn create(
        name: String,
        kind: SymbolKind,
        range: Range,
        uri: Option<String>,
        container_name: Option<String>,
    ) -> Self {
        SymbolInformation {
            name,
            kind,
            deprecated: None,
            location: Location {
                uri: match uri {
                    Some(uri) => uri,
                    _ => "".to_string(),
                },
                range,
            },
            container_name,
        }
    }
}
