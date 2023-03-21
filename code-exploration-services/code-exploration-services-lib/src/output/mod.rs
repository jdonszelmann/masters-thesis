use crate::{Analysis, SourceCode};

mod html;


pub trait Annotater {
    type Output;

    fn annotate(&self, s: &SourceCode, a: Analysis) -> Self::Output;
}
