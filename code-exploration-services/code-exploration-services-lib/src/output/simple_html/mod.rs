use crate::analysis::dir::{Analysis, GetAnalysisError};
use crate::analysis::field::Field;
use crate::output::simple_html::themes::sanitize_theme_name;
use crate::output::simple_html::tokenize::OutlineSetting::GenerateOutline;
use crate::output::Annotater;
use crate::sources::dir::{ContentsError, SourceDir};
use crate::sources::span::Span;
use crate::textmate::theme::TextmateThemeManager;
use std::collections::HashMap;
use std::path::PathBuf;
use themes::ScopeSelectorFromStrError;
use thiserror::Error;

mod generate_html;
mod outline;
mod themes;
mod tokenize;

pub struct SimpleHtml;

#[derive(Debug, Error)]
pub enum SimpleHtmlError {
    #[error("parsing scope selector")]
    ParseScopeSelector(#[from] ScopeSelectorFromStrError),

    #[error("contents")]
    Contents(#[from] ContentsError),

    #[error("get analysis for file {1:?}")]
    GetAnalysis(#[source] GetAnalysisError, PathBuf),
}

pub enum IndexField<'a> {
    Field(&'a Field),
    ReferenceTarget,
}

type FieldIndex<'a> = HashMap<usize, Vec<(&'a Span, IndexField<'a>)>>;

impl Annotater for SimpleHtml {
    type Output = Result<String, SimpleHtmlError>;

    fn annotate(&self, source: &SourceDir, a: Analysis) -> Self::Output {
        let file = source.files().next().expect("one source file");
        let a = a
            .analysis_for(file)
            .map_err(|i| SimpleHtmlError::GetAnalysis(i, file.path().to_path_buf()))?;

        let themes = TextmateThemeManager::default();

        let field_index = tokenize::index_analysis(&a);
        let tokens = tokenize::tokenize_string(&file.contents()?, 0, &field_index, GenerateOutline);
        let outline = outline::generate_outline(&a, &field_index, file)?;

        let style = include_str!("./style.css");
        let script = include_str!("./script.js");
        let themes_css = themes::generate_theme_styles(&themes)?;

        generate_html::generate_html(themes, tokens, outline, style, script, themes_css, file)
    }
}
