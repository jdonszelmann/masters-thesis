use crate::{Analysis, SourceCode};

pub mod simple_html;

pub trait Annotater {
    type Output;

    fn annotate(&self, s: &SourceCode, a: Analysis) -> Self::Output;
}
