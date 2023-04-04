use crate::analysis::Field;
use crate::analysis::Span;
use crate::input::{Analyser, AnalysisError};
use crate::{Analysis, SourceCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TextmateAnalysisError {
    #[error("parse textmate: {0}")]
    Textmate(#[from] textmate::ParseError),
}

pub struct TextmateAnalyser;

impl Analyser for TextmateAnalyser {
    fn syntax_coloring(&self, s: &SourceCode) -> Result<Analysis, AnalysisError> {
        let Some(ext) = s.extension() else {
            return Err(AnalysisError::NotImplemented);
        };
        let Some(parser) = textmate::TextmateGrammar::from_language(ext) else {
            return Err(AnalysisError::NotImplemented)
        };

        let res = parser
            .parse(s.as_str())
            .map_err(TextmateAnalysisError::Textmate)?;
        let mut fields = Vec::new();

        for (span, name) in res {
            fields.push((
                Span::new(span.start, span.len),
                Field::SyntaxColour(name.to_string()),
            ))
        }

        Ok(Analysis::new(s, fields))
    }
}
