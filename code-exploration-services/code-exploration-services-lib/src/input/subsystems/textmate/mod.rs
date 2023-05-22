use crate::input::{Analyser, AnalysisError};
use thiserror::Error;
use crate::analysis::dir::Analysis;
use crate::analysis::field::Field;
use crate::analysis::file::FileAnalysis;
use crate::sources::dir::SourceDir;
use crate::sources::span::Span;
use crate::textmate;

#[derive(Debug, Error)]
pub enum TextmateAnalysisError {
    #[error("parse textmate")]
    Textmate(#[from] textmate::ParseError),
}

pub struct TextmateAnalyser;

impl Analyser for TextmateAnalyser {
    fn syntax_coloring<'a>(&self, s: &'a SourceDir) -> Result<Analysis, AnalysisError> {
        s.map_analyze(|file| {
            let Some(ext) = file.path().extension() else {
                return Err(AnalysisError::NotImplemented);
            };
            let Some(parser) = textmate::TextmateGrammar::from_language(&ext.to_string_lossy())? else {
                return Err(AnalysisError::NotImplemented)
            };

            let res = parser
                .parse(file.contents()?.as_str())
                .map_err(TextmateAnalysisError::Textmate)?;
            let mut fields = Vec::new();

            for (span, name) in res {
                fields.push((
                    Span::new(span.start, span.len),
                    Field::SyntaxColour(name.to_string()),
                ))
            }

            Ok(FileAnalysis::new(file, fields)?)
        })
    }
}
