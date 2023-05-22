use crate::analysis::dir::Analysis;
use crate::sources::dir::SourceDir;

pub mod simple_html;

pub trait Annotater {
    type Output;

    fn annotate(&self, s: &SourceDir, a: Analysis) -> Self::Output;
}
