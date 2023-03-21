use crate::input::{Analyser, AnalysisError};
use crate::{Analysis, SourceCode};

pub struct TextmateAnalyser;

impl Analyser for TextmateAnalyser {
    fn syntax_coloring(&self, s: &SourceCode) -> Result<Analysis, AnalysisError> {
        // textmate::TextmateGrammar::from_json()
        todo!()
    }
}