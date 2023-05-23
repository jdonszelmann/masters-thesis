use crate::analysis::dir::Analysis;
use crate::analysis::file::NewFileAnalysisError;
use crate::input::subsystems::ctags::CtagsAnalysisError;
use crate::input::subsystems::lsp::LanguageServerError;
use crate::input::subsystems::textmate::TextmateAnalysisError;
use crate::sources::dir::{ContentsError, SourceDir};
use crate::textmate::grammar::FromLanguageError;
use thiserror::Error;

pub mod subsystems;

#[derive(Error, Debug)]
pub enum AnalysisError {
    /// Returned when an operation is not implemented by an analyser.
    /// The next one is tried.
    #[error("not implemented")]
    NotImplemented,

    #[error("contents")]
    Contents(#[from] ContentsError),

    #[error("create file analysis structure")]
    NewAnalysis(#[from] NewFileAnalysisError),

    #[error("ctags")]
    Ctags(#[from] CtagsAnalysisError),

    #[error("textmate")]
    TextMate(#[from] TextmateAnalysisError),

    #[error("language server")]
    Lsp(#[from] LanguageServerError),

    #[error("parse grammar")]
    ParseGrammar(#[from] FromLanguageError),
}

#[inline]
fn or<T>(fs: &[&dyn Fn() -> Result<T, AnalysisError>]) -> Result<T, AnalysisError> {
    for f in fs {
        match f() {
            Ok(i) => return Ok(i),
            Err(AnalysisError::NotImplemented) => continue,
            Err(e) => return Err(e),
        }
    }
    Err(AnalysisError::NotImplemented)
}

macro_rules! define_analysis_chain {
    ($s: ident, [$name: ident, $($others: ident),*] for [$($path: path),*] ) => {
        match (or(&[$(&|| $path.$name($s)),*]), || define_analysis_chain!($s, [$($others),*] for [$($path),*]),) {
            (Ok(i), f) => {
                match f() {
                    Ok(j) => Ok(i.merge(j).expect("from same source file so should never fail")),
                    Err(AnalysisError::NotImplemented) => Ok(i),
                    Err(e) => Err(e),
                }
            },
            (Err(AnalysisError::NotImplemented), f) => f(),
            (Err(e), _) => Err(e)
        }
    };
    ($s: ident, [$name: ident] for [$($path: path),*] ) => {
        or(&[$(
            &|| $path.$name($s)
        ),*])
    };
}

macro_rules! define_analysis_types {
    ($($name: ident),* $(,)?; for $($path: path),* $(,)? ) => {
        pub trait Analyser {
        $(
            fn $name<'a>(&self, _s: &SourceDir) -> Result<Analysis, AnalysisError> { Err(AnalysisError::NotImplemented) }
        )*
        }

        pub fn analyse(s: &SourceDir) -> Result<Analysis, AnalysisError> {
            define_analysis_chain!(s, [$($name),*] for [$($path),*])
        }
    };
}

define_analysis_types!(
    syntax_coloring,
    code_folding,
    outline,
    symbol_navigation,
    hover_documentation,
    signature_help,
    expansions,
    diagnostic_messages;
    for
    subsystems::lsp::LspAnalyser,
    subsystems::textmate::TextmateAnalyser,
    subsystems::ctags::CtagsAnalyser,
);
