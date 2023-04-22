use super::CompletionItem;

/// Represents a collection of [completion items](#CompletionItem) to be presented
/// in the editor.
#[derive(Debug, serde::Serialize)]
pub struct CompletionList {
    /// This list it not complete. Further typing results in recomputing this list.
    pub is_incomplete: bool,

    /// The completion items.
    pub items: Vec<CompletionItem>,
}

/// The CompletionList namespace provides functions to deal with
/// completion lists.
impl CompletionList {
    /// Creates a new completion list.
    ///
    /// @param items The completion items.
    /// @param isIncomplete The list is not complete.
    pub fn create(items: Option<Vec<CompletionItem>>, is_incomplete: Option<bool>) -> Self {
        CompletionList {
            items: match items {
                Some(items) => items,
                None => Vec::new(),
            },
            is_incomplete: match is_incomplete {
                Some(is_incomplete) => is_incomplete,
                None => false,
            },
        }
    }
}
