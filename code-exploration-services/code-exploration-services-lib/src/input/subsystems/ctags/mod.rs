use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;
use strum::ParseError;
use crate::input::{Analyser, AnalysisError};
use crate::{Analysis, SourceCode};
use thiserror::Error;
use crate::analysis::Field;

pub mod tags;
pub mod xrefs;
pub mod xref_kinds;

#[derive(Error, Debug)]
pub enum CtagsAnalysisError {
    #[error("runnning ctags command: {0}")]
    RunCtagsCommand(io::Error),

    #[error("ctags error: {0}")]
    Ctags(String),

    #[error("parse tag: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("parse xref: {0}")]
    ParseXref(String),

    #[error("can't parse '{1}' as Xref kind: {0}")]
    ParseXrefKind(ParseError, String),

    #[error("can't parse as int")]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub struct CtagsAnalyser;

impl Analyser for CtagsAnalyser {
    fn outline(&self, s: &SourceCode) -> Result<Analysis, AnalysisError> {
        let xref_output = xrefs::run_xref(s)?;

        let mut index = HashMap::new();
        for xref in &xref_output.xrefs {
            index.entry((&xref.name, &xref.kind)).or_insert_with(Vec::new).push(xref);
        }

        let mut res = Vec::new();
        for xref in &xref_output.xrefs {
            let span = xref.span(s);
            let parent = if let (Some(parent), Some(parent_kind)) = (&xref.parent, &xref.parent_kind) {
                index.get(&(parent, parent_kind))
                    .iter()
                    .filter_map(|i| i.iter().min_by_key(|&&i| {
                        let m = i.span(s).midpoint();
                        let sm = span.midpoint();

                        if sm > m {
                            usize::MAX
                        } else {
                            m - sm
                        }
                    }).map(|i| i.span(s)))
                    .next()
            } else {
                None
            };

            res.push((
                span,
                Field::Outline {
                    description: Some(xref.kind.as_ref().to_string()),
                    parent,
                }
            ));
        }

        Ok(Analysis::new(s, res))
    }
}

