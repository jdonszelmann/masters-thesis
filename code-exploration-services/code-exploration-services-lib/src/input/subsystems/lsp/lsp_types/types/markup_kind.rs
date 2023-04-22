/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. This kinds
/// are reserved for internal usage.
#[derive(Debug, serde::Serialize)]
pub enum MarkupKind {
    /// Plain text is supported as a content format
    PlainText,

    /// Markdown is supported as a content format
    Markdown,
}
