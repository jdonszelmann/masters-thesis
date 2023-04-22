/// A document filter denotes a document by different properties like
/// the [language](#TextDocument.languageId), the [scheme](#Uri.scheme) of
/// its resource, or a glob-pattern that is applied to the [path](#TextDocument.fileName).
///
/// @sample A language filter that applies to typescript files on disk: `{ language: 'typescript', scheme: 'file' }`
/// @sample A language filter that applies to all package.json paths: `{ language: 'json', pattern: '**package.json' }`
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DocumentFilter {
    /// A language id, like `typescript`.
    pub language: Option<String>,

    /// A Uri [scheme](#Uri.scheme), like `file` or `untitled`.
    pub scheme: Option<String>,

    /// A glob pattern, like `*.{ts,js}`.
    pub pattern: Option<String>,
}
