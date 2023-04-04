use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;
use std::str::FromStr;
use axohtml::dom::DOMTree;
use axohtml::{html, text, unsafe_text};
use axohtml::elements::{PhrasingContent, source, span, style};
use axohtml::types::{Class, SpacedSet};
use itertools::Itertools;
use thiserror::Error;
use typed_arena::Arena;
use textmate::theme::{FontStyle, Settings, SettingsItem, TextmateTheme, TextmateThemeManager};
use themes::ScopeSelectorFromStrError;
use crate::SourceCode;
use crate::analysis::{Analysis, Field, Span};
use crate::output::Annotater;
use crate::output::simple_html::themes::sanitize_theme_name;
use crate::output::simple_html::format::format_tokens;

mod tokenize;
mod outline;
mod format;
mod themes;

pub struct SimpleHtml;

#[derive(Debug, Error)]
pub enum SimpleHtmlError {
    #[error("parsing scope selector: {0}")]
    ParseScopeSelector(#[from] ScopeSelectorFromStrError)
}

type FieldIndex<'a> = HashMap<usize, Vec<(&'a Span, &'a Field)>>;
type Classes = Vec<String>;

impl Annotater for SimpleHtml {
    type Output = Result<String, SimpleHtmlError>;

    fn annotate(&self, source: &SourceCode, a: Analysis) -> Self::Output {
        let themes = TextmateThemeManager::default();

        let field_index = tokenize::index_analysis(&a);
        let tokens = tokenize::tokenize_string(source.as_str(), 0, &field_index);
        let outline = outline::generate_outline(&a, &field_index, &source)?;

        let style = include_str!("./style.css");
        let script = include_str!("./script.js");
        let themes_css = themes::generate_theme_styles(&themes)?;

        let doc: DOMTree<String> = html! {
            <html>
                <head>
                    <title>"CES"</title>
                </head>
                <body>
                    <div id="main" class={SpacedSet::try_from(["change-theme", sanitize_theme_name(&themes.iter().next().unwrap().name).as_str()]).unwrap()}>
                        {unsafe_text!("<style>{}</style>", style)}
                        {unsafe_text!("<style>{}</style>", themes_css)}

                        <div class="theme">
                            <select id="change-theme">
                                {
                                    themes.iter().map(|i| html! {
                                        <option value={sanitize_theme_name(&i.name)}>{text!("{}", i.name)}</option>
                                    })
                                }
                            </select>
                        </div>
                        <div class="outline">
                            <pre>
                            {unsafe_text!("{}", outline)}
                            </pre>
                        </div>
                        <div class="code">
                            <pre>
                                {
                                    format_tokens(tokens)
                                }
                            </pre>
                        </div>
                        <script>
                            {unsafe_text!("{}", script)}
                        </script>
                    </div>
                </body>
            </html>
        };

        Ok(format!("<!DOCTYPE html>\n{}\n", doc.to_string()))
    }
}

enum ScopeSelector {
    Selector(String),
    Or(Vec<ScopeSelector>),
    Inside(String, Box<ScopeSelector>),
}

impl FromStr for ScopeSelector {
    type Err = ScopeSelectorFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::root_from_str(s)
    }
}

impl ScopeSelector {
    pub fn classes<'a>(&'a self) -> Box<dyn Iterator<Item=String> + 'a> {
        match self {
            ScopeSelector::Selector(s) => Box::new(iter::once(format!(".{s}"))),
            ScopeSelector::Or(items) => {
                Box::new(items.iter().map(|i| i.classes()).flatten())
            }
            ScopeSelector::Inside(a, b) => {
                Box::new(b.classes().map(move |i| {
                    format!(".{a}{i}")
                }))
            }
        }
    }

    fn root_from_str(s: &str) -> Result<Self, ScopeSelectorFromStrError> {
        let parts = s.split(",").collect::<Vec<_>>();
        if parts.len() == 0 {
            Err(ScopeSelectorFromStrError::Empty(s.to_string()))
        } else if parts.len() == 1 {
            Self::inside_from_str(parts[0])
        } else {
            let mut res = Vec::new();
            for i in parts {
                res.push(ScopeSelector::from_str(i)
                    .map_err(|e| {
                        ScopeSelectorFromStrError::Inside(
                            s.replace(".", "-"),
                            Box::new(e)
                        )
                    })?);
            }

            Ok(Self::Or(res))
        }
    }

    fn inside_from_str(s: &str) -> Result<Self, ScopeSelectorFromStrError> {
        let parts = s.splitn(2, " ").filter(|i| !i.trim().is_empty()).collect::<Vec<_>>();
        if parts.len() == 0 {
            Err(ScopeSelectorFromStrError::Empty(s.to_string()))
        } else if parts.len() == 1 {
            Ok(Self::Selector(parts[0].replace(".", "-")))
        } else {
            Ok(Self::Inside(
                parts[0].trim().replace(".", "-"),
                Box::new(
                    Self::inside_from_str(parts[1].trim())
                        .map_err(|e| {
                            ScopeSelectorFromStrError::Inside(
                                s.to_string(),
                                Box::new(e)
                            )
                        })?
                )
            ))
        }
    }
}

