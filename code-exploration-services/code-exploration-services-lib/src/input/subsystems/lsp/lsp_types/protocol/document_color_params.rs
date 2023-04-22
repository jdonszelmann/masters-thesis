use crate::input::subsystems::lsp::lsp_types::types::TextDocumentIdentifier;

/// Parameters for a [DocumentColorRequest](#DocumentColorRequest).
pub struct DocumentColorParams {
	/// The text document.
	pub text_document: TextDocumentIdentifier,
}
