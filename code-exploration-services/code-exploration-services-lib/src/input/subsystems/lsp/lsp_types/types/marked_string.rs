use regex::Regex;

/// MarkedString can be used to render human readable text. It is either a markdown string
/// or a code-block that provides a language and a code snippet. The language identifier
/// is semantically equal to the optional language identifier in fenced code blocks in GitHub
/// issues. See https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// The pair of a language and a value is an equivalent to markdown:
/// ```${language}
/// ${value}
/// ```
///
/// Note that markdown strings will be sanitized - that means html will be escaped.
/// @deprecated use MarkupContent instead.
#[derive(Debug, serde::Serialize)]
pub struct MarkedString {
    pub language: String,

    pub value: String,
}

impl MarkedString {
    /// Creates a marked string from plain text.
    ///
    /// @param plainText The plain text.
    pub fn from_plain_text(plain_text: String) -> String {
        Regex::new(r"[\\`*_{}[\]()#+\-.!]").unwrap().replace_all(&plain_text, "\\$&").to_string() // escape markdown syntax tokens: http://daringfireball.net/projects/markdown/syntax#backslash
    }
}
