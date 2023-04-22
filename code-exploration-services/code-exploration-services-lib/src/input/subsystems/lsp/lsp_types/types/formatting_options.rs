use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union3;
use std::collections::HashMap;

/// Value-object describing what options formatting should use.
#[derive(Debug, serde::Serialize)]
pub struct FormattingOptions {
    /// Size of a tab in spaces.
    pub tab_size: i32,

    /// Prefer spaces over tabs.
    pub insert_spaces: bool,

    /// Signature for further properties.
    pub properties: Option<HashMap<String, Union3<bool, f64, String>>>,
}

/// The FormattingOptions namespace provides helper functions to work with
/// [FormattingOptions](#FormattingOptions) literals.
impl FormattingOptions {
    /// Creates a new FormattingOptions literal.
    pub fn create(tab_size: i32, insert_spaces: bool) -> Self {
        FormattingOptions {
            tab_size,
            insert_spaces,
            properties: None,
        }
    }
}
