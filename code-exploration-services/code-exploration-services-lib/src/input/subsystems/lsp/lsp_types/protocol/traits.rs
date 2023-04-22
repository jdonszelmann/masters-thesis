pub trait TextDocumentRegistrationOptions {}

impl TextDocumentRegistrationOptions for super::TextDocumentRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::TextDocumentChangeRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::TextDocumentSaveRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::CompletionRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::SignatureHelpRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::CodeLensRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::DocumentOnTypeFormattingRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::DocumentLinkRegistrationOptions {}
impl TextDocumentRegistrationOptions for super::ExecuteCommandRegistrationOptions {}

pub trait TextDocumentPositionParams {}

impl TextDocumentPositionParams for super::CompletionParams {}
