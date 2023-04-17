use crate::analysis::Field;
use crate::input::{Analyser, AnalysisError};
use crate::{Analysis, SourceCode};
use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;
use strum::ParseError;
use thiserror::Error;

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

pub struct RustdocAnalyser;

impl Analyser for RustdocAnalyser {
}

