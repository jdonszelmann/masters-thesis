use crate::SourceCode;
use crate::analysis::Analysis;
use crate::output::Annotater;

struct Html;

impl Annotater for Html {
    type Output = ();

    fn annotate(&self, _s: &SourceCode, _a: Analysis) -> Self::Output {
        todo!()
    }
}