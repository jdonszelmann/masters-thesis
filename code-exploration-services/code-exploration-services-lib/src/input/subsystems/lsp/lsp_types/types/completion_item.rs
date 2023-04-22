use super::{Command, CompletionItemKind, InsertTextFormat, MarkupContent, TextEdit};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Union, Value};

/// A completion item represents a text snippet that is
/// proposed to complete text that is being typed.
#[derive(Debug, serde::Serialize)]
pub struct CompletionItem {
    /// The label of this completion item. By default
    /// also the text that is inserted when selecting
    /// this completion.
    pub label: String,

    /// The kind of this completion item. Based of the kind
    /// an icon is chosen by the editor.
    pub kind: Option<CompletionItemKind>,

    /// A human-readable string with additional information
    /// about this item, like type or symbol information.
    pub detail: Option<String>,

    /// A human-readable string that represents a doc-comment.
    pub documentation: Option<Union<String, MarkupContent>>,

    /// Indicates if this item is deprecated.
    pub deprecated: Option<bool>,

    /// Select this item when showing.
    ///
    /// *Note* that only one completion item can be selected and that the
    /// tool / client decides which item that is. The rule is that the///first*
    /// item of those that match best is selected.
    pub preselect: Option<bool>,

    /// A string that should be used when comparing this item
    /// with other items. When `falsy` the [label](#CompletionItem.label)
    /// is used.
    pub sort_text: Option<String>,

    /// A string that should be used when filtering a set of
    /// completion items. When `falsy` the [label](#CompletionItem.label)
    /// is used.
    pub filter_text: Option<String>,

    /// A string that should be inserted into a document when selecting
    /// this completion. When `falsy` the [label](#CompletionItem.label)
    /// is used.
    ///
    /// The `insertText` is subject to interpretation by the client side.
    /// Some tools might not take the string literally. For example
    /// VS Code when code complete is requested in this example `con<cursor position>`
    /// and a completion item with an `insertText` of `console` is provided it
    /// will only insert `sole`. Therefore it is recommended to use `textEdit` instead
    /// since it avoids additional client side interpretation.
    ///
    /// @deprecated Use textEdit instead.
    pub insert_text: Option<String>,

    /// The format of the insert text. The format applies to both the `insertText` property
    /// and the `newText` property of a provided `textEdit`.
    pub insert_text_format: Option<InsertTextFormat>,

    /// An [edit](#TextEdit) which is applied to a document when selecting
    /// this completion. When an edit is provided the value of
    /// [insertText](#CompletionItem.insertText) is ignored.
    ///
    /// *Note:* The text edit's range must be a [single line] and it must contain the position
    /// at which completion has been requested.
    pub text_edit: Option<TextEdit>,

    /// An optional array of additional [text edits](#TextEdit) that are applied when
    /// selecting this completion. Edits must not overlap (including the same insert position)
    /// with the main [edit](#CompletionItem.textEdit) nor with themselves.
    ///
    /// Additional text edits should be used to change text unrelated to the current cursor position
    /// (for example adding an import statement at the top of the file if the completion item will
    /// insert an unqualified type).
    pub additional_text_edits: Option<Vec<TextEdit>>,

    /// An optional set of characters that when pressed while this completion is active will accept it first and
    /// then type that character.///Note* that all commit characters should have `length=1` and that superfluous
    /// characters will be ignored.
    pub commit_characters: Option<Vec<String>>,

    /// An optional [command](#Command) that is executed///after* inserting this completion.///Note* that
    /// additional modifications to the current document should be described with the
    /// [additionalTextEdits](#CompletionItem.additionalTextEdits)-property.
    pub command: Option<Command>,

    /// An data entry field that is preserved on a completion item between
    /// a [CompletionRequest](#CompletionRequest) and a [CompletionResolveRequest]
    /// (#CompletionResolveRequest)
    pub data: Option<Value>,
}

/// The CompletionItem namespace provides functions to deal with
/// completion items.
impl CompletionItem {
    /// Create a completion item and seed it with a label.
    /// @param label The completion item's label
    pub fn create(label: String) -> Self {
        CompletionItem {
            label,
            kind: None,
            detail: None,
            documentation: None,
            deprecated: None,
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: None,
            insert_text_format: None,
            text_edit: None,
            additional_text_edits: None,
            commit_characters: None,
            command: None,
            data: None,
        }
    }
}
