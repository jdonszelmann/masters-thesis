pub mod traits;

mod position;
mod range;
mod location;
mod color;
mod color_information;
mod color_presentation;
mod diagnostic_related_information;
mod diagnostic_severity;
mod diagnostic;
mod command;
mod text_edit;
mod text_document_edit;
mod workspace_edit;
mod text_document_identifier;
mod versioned_text_document_identifier;
mod text_document_item;
mod markup_kind;
mod markup_content;
mod completion_item_kind;
mod insert_text_format;
mod completion_item;
mod completion_list;
mod marked_string;
mod hover;
mod parameter_information;
mod signature_information;
mod signature_help;
mod definition;
mod reference_context;
mod document_highlight_kind;
mod document_highlight;
mod symbol_kind;
mod symbol_information;
mod document_symbol;
mod document_symbol_params;
mod workspace_symbol_params;
mod code_action_kind;
mod code_action_context;
mod code_action;
mod code_lens;
mod formatting_options;
mod document_link;
mod text_document_save_reason;
mod text_document_content_change_event;

pub use self::position::Position;
pub use self::range::Range;
pub use self::location::Location;
pub use self::color::Color;
pub use self::color_information::ColorInformation;
pub use self::color_presentation::ColorPresentation;
pub use self::diagnostic_related_information::DiagnosticRelatedInformation;
pub use self::diagnostic_severity::DiagnosticSeverity;
pub use self::diagnostic::Diagnostic;
pub use self::command::Command;
pub use self::text_edit::TextEdit;
pub use self::text_document_edit::TextDocumentEdit;
pub use self::workspace_edit::WorkspaceEdit;
pub use self::text_document_identifier::TextDocumentIdentifier;
pub use self::versioned_text_document_identifier::VersionedTextDocumentIdentifier;
pub use self::text_document_item::TextDocumentItem;
pub use self::markup_kind::MarkupKind;
pub use self::markup_content::MarkupContent;
pub use self::completion_item_kind::CompletionItemKind;
pub use self::insert_text_format::InsertTextFormat;
pub use self::completion_item::CompletionItem;
pub use self::completion_list::CompletionList;
pub use self::marked_string::MarkedString;
pub use self::hover::Hover;
pub use self::parameter_information::ParameterInformation;
pub use self::signature_information::SignatureInformation;
pub use self::signature_help::SignatureHelp;
pub use self::definition::Definition;
pub use self::reference_context::ReferenceContext;
pub use self::document_highlight_kind::DocumentHighlightKind;
pub use self::document_highlight::DocumentHighlight;
pub use self::symbol_kind::SymbolKind;
pub use self::symbol_information::SymbolInformation;
pub use self::document_symbol::DocumentSymbol;
pub use self::document_symbol_params::DocumentSymbolParams;
pub use self::workspace_symbol_params::WorkspaceSymbolParams;
pub use self::code_action_kind::CodeActionKind;
pub use self::code_action_context::CodeActionContext;
pub use self::code_action::CodeAction;
pub use self::code_lens::CodeLens;
pub use self::formatting_options::FormattingOptions;
pub use self::document_link::DocumentLink;
pub use self::text_document_save_reason::TextDocumentSaveReason;
pub use self::text_document_content_change_event::TextDocumentContentChangeEvent;

pub const EOL: [&'static str; 3] = ["\n", "\r\n", "\r"];