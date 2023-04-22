use crate::analysis::{Analysis, Field, Span};
use crate::output::simple_html::themes::sanitize_theme_name;
use crate::output::Annotater;
use crate::SourceCode;
use std::collections::HashMap;
use textmate::theme::TextmateThemeManager;
use themes::ScopeSelectorFromStrError;
use thiserror::Error;
use crate::output::simple_html::tokenize::OutlineSetting::GenerateOutline;

mod generate_html;
mod outline;
mod themes;
mod tokenize;

pub struct SimpleHtml;

#[derive(Debug, Error)]
pub enum SimpleHtmlError {
    #[error("parsing scope selector")]
    ParseScopeSelector(#[from] ScopeSelectorFromStrError),
}

type FieldIndex<'a> = HashMap<usize, Vec<(&'a Span, &'a Field)>>;

impl Annotater for SimpleHtml {
    type Output = Result<String, SimpleHtmlError>;

    fn annotate(&self, source: &SourceCode, a: Analysis) -> Self::Output {
        let themes = TextmateThemeManager::default();

        let field_index = tokenize::index_analysis(&a);
        let tokens = tokenize::tokenize_string(source.as_str(), 0, &field_index, GenerateOutline);
        let outline = outline::generate_outline(&a, &field_index, source)?;

        let style = include_str!("./style.css");
        let script = include_str!("./script.js");
        let themes_css = themes::generate_theme_styles(&themes)?;

        generate_html::generate_html(themes, tokens, outline, style, script, themes_css)
    }
}
