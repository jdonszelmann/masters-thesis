use super::TextEdit;

#[derive(Debug, serde::Serialize)]
pub struct ColorPresentation {
    /// The label of this color presentation. It will be shown on the color
    /// picker header. By default this is also the text that is inserted when selecting
    /// this color presentation.
    pub label: String,

    /// An [edit](#TextEdit) which is applied to a document when selecting
    /// this presentation for the color.  When `falsy` the [label](#ColorPresentation.label)
    /// is used.
    pub text_edit: Option<TextEdit>,

    /// An optional array of additional [text edits](#TextEdit) that are applied when
    /// selecting this color presentation. Edits must not overlap with the main [edit](#ColorPresentation.textEdit) nor with themselves.
    pub additional_text_edits: Option<Vec<TextEdit>>,
}

/// The Color namespace provides helper functions to work with
/// [ColorPresentation](#ColorPresentation) literals.
impl ColorPresentation {
    /// Creates a new ColorInformation literal.
    pub fn create(
        label: String,
        text_edit: Option<TextEdit>,
        additional_text_edits: Option<Vec<TextEdit>>,
    ) -> Self {
        ColorPresentation {
            label,
            text_edit,
            additional_text_edits,
        }
    }
}
