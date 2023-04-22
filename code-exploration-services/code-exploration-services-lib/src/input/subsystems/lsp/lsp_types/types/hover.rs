use super::{MarkedString, MarkupContent, Range};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union3;

/// The result of a hover request.
#[derive(Debug, serde::Serialize)]
pub struct Hover {
    /// The hover's content
    pub contents: Union3<MarkupContent, MarkedString, Vec<MarkedString>>,

    /// An optional range
    pub range: Option<Range>,
}
