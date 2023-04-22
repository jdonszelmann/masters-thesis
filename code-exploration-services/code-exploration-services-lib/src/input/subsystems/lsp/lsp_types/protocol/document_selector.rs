use super::DocumentFilter;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

/// A document selector is the combination of one or many document filters.
///
/// @sample `let sel:DocumentSelector = [{ language: 'typescript' }, { language: 'json', pattern: '**∕tsconfig.json' }]`;
pub type DocumentSelector = Vec<Union<String, DocumentFilter>>;
