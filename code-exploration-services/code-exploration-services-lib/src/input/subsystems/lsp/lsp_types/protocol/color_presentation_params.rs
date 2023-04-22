use crate::input::subsystems::lsp::lsp_types::types::{Color, Range, TextDocumentIdentifier};

/// Parameters for a [ColorPresentationRequest](#ColorPresentationRequest).
pub struct ColorPresentationParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,

	/// The color to request presentations for.
	pub color: Color,

	/// The range where the color would be inserted. Serves as a context.
	pub range: Range,
}
