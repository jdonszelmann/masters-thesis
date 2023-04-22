use super::{Range, SymbolKind};

/// Represents programming constructs like variables, classes, interfaces etc.
/// that appear in a document. Document symbols can be hierarchical and they
/// have two ranges: one that encloses its definition and one that points to
/// its most interesting range, e.g. the range of an identifier.
#[derive(Debug, serde::Serialize)]
pub struct DocumentSymbol {
    /// The name of this symbol.
    pub name: String,

    /// More detail for this symbol, e.g the signature of a function. If not provided
    /// the name is used.
    pub detail: Option<String>,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    /// Indicates if this symbol is deprecated.
    pub deprecated: Option<bool>,

    /// The range enclosing this symbol not including leading/trailing whitespace but everything else
    /// like comments. This information is typically used to determine if the the clients cursor is
    /// inside the symbol to reveal in the symbol in the UI.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being picked, e.g the name of a function.
    /// Must be contained by the the `range`.
    pub selection_range: Range,

    /// Children of this symbol, e.g. properties of a class.
    pub children: Option<Vec<DocumentSymbol>>,
}

impl DocumentSymbol {
    /// Creates a new symbol information literal.
    ///
    /// @param name The name of the symbol.
    /// @param detail The detail of the symbol.
    /// @param kind The kind of the symbol.
    /// @param range The range of the symbol.
    /// @param selectionRange The selectionRange of the symbol.
    /// @param children Children of the symbol.
    pub fn create(
        name: String,
        detail: Option<String>,
        kind: SymbolKind,
        range: Range,
        selection_range: Range,
        children: Option<Vec<DocumentSymbol>>,
    ) -> Self {
        DocumentSymbol {
            name,
            detail,
            kind,
            deprecated: None,
            range,
            selection_range,
            children,
        }
    }
}
