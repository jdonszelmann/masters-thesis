use crate::analysis::dir::Analysis;
use crate::sources::dir::SourceDir;
use crate::sources::span::Span;

pub mod simple_html;
pub mod latex;
pub mod tokenize;
pub mod scope_selector;
pub mod theme;

pub trait Annotater {
    type Output;
    type Params: Default;

    fn annotate(&self, s: &SourceDir, a: Analysis, params: Self::Params) -> Self::Output;
}

pub fn span_to_class(span: &Span) -> String {
    fn span_to_class_helper(span: &Span) -> String {
        if let Some(ref i) = span.next {
            format!("{}-{}-{}", span.start, span.len, span_to_class_helper(i))
        } else {
            format!("{}-{}", span.start, span.len)
        }
    }

    format!("goto-{}", span_to_class_helper(span))
}
