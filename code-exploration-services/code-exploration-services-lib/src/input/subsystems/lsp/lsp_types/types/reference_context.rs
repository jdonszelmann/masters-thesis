/// Value-object that contains additional information when
/// requesting references.
#[derive(Debug, serde::Serialize)]
pub struct ReferenceContext {
    /// Include the declaration of the current symbol.
    pub include_declaration: bool,
}
