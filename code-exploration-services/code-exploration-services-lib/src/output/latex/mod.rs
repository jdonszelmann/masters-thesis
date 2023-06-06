use std::collections::HashSet;
use std::path::PathBuf;
use itertools::Itertools;
use onig::EncodedChars;
use thiserror::Error;
use tracing::info;
use crate::analysis::dir::{Analysis, GetAnalysisError};
use crate::Annotater;
use crate::output::latex::colors::{Colors, ResolveColorError};
use crate::output::scope_selector::ScopeSelectorFromStrError;
use crate::output::span_to_class;
use crate::output::theme::Theme;
use crate::output::tokenize::{index_analysis, OutlineSetting, Token, tokenize_string};
use crate::sources::dir::{ContentsError, SourceDir};
use crate::textmate::theme::TextmateThemeManager;

mod colors;

pub struct Latex;

#[derive(Debug, Error)]
pub enum LatexError {
    #[error("get analysis for file {1:?}")]
    GetAnalysis(#[source] GetAnalysisError, PathBuf),

    #[error("contents")]
    Contents(#[from] ContentsError),

    #[error("unknown theme: {0}")]
    UnknownTheme(String),

    #[error("parse scope selector")]
    ScopeSelectorFromTheme(#[from] ScopeSelectorFromStrError),

    #[error("resolve color in theme")]
    ResolveColor(#[from] ResolveColorError),
}

pub struct LatexOutput {
    pub latex_source: (PathBuf, String),
    pub codex_sty: (PathBuf, String),
}

pub struct LatexParams {
    theme: String,
}

impl Default for LatexParams {
    fn default() -> Self {
        Self {
            theme: "One Dark".to_string(),
        }
    }
}

impl Annotater for Latex {
    type Output = Result<LatexOutput, LatexError>;
    type Params = LatexParams;

    fn annotate(&self, source: &SourceDir, a: Analysis, params: LatexParams) -> Self::Output {
        let file = source.files().next().expect("one source file");
        let a = a
            .analysis_for(file)
            .map_err(|i| LatexError::GetAnalysis(i, file.path().to_path_buf()))?;

        let field_index = index_analysis(&a);
        let tokens = tokenize_string(&file.contents()?, 0, &field_index, OutlineSetting::DontGenerateOutline);

        let themes = TextmateThemeManager::default();

        let Some(theme) = themes.iter().find(|i| i.name == params.theme) else {
            return Err(LatexError::UnknownTheme(params.theme.clone()));
        };
        let theme = Theme::from_textmate(theme)?;

        let colors = Colors::from_tokens(&tokens, &theme)?;
        let latex = generate_latex(&tokens, &colors);
        let color_definitions = colors.definitions();

        Ok(LatexOutput {
            latex_source: (PathBuf::from("output.tex"), format!(r#"
{color_definitions}
\begin{{codexcode}}
    {latex}
\end{{codexcode}}
            "#)),
            codex_sty: (PathBuf::from("codex.sty"), include_str!("codex.sty").to_string()),
        })
    }
}

fn validate(inp: impl AsRef<str>) -> String {
    inp.as_ref()
        .replace("\\", r"\\")
        .replace("#", r"\#")
        .replace("$", r"\$")
        .replace("%", r"\%")
        .replace("&", r"\&")
        .replace("{", r"\{")
        .replace("}", r"\}")
        .replace(" ", r"\char0032 \allowbreak ")
        .replace("\t", r"\char0009 ")
        .replace("_", r"\_")
        .replace("~", r"\textasciicircum")
        .replace("^", r"\textasciicircum")
        .to_string()
}

fn generate_latex(tokens: &[Token], colors: &Colors) -> String {
    let mut res = Vec::<String>::new();
    let mut labels_already_inserted = HashSet::new();
    let mut all_labels = HashSet::new();

    for i in tokens {
        if let Token::Token { classes , ..} = i {
            for i in &classes.reference_targets {
                all_labels.insert(i);
            }
        }
    }

    for i in tokens {
        match i {
            Token::Token { text, classes } => {
                if text.is_empty() {
                    continue
                }

                let mut refs = Vec::new();
                let mut labels = Vec::new();
                for i in &classes.reference_targets {
                    if !labels_already_inserted.contains(&i) {
                        labels_already_inserted.insert(i);
                        labels.push(i.replace("-", ""));
                    }
                }
                let mut refs_already_inserted = HashSet::new();
                for i in &classes.references {
                    let label = span_to_class(&i.to);
                    if all_labels.contains(&label) && !refs_already_inserted.contains(&label) {
                        refs.push((
                            label.replace("-", ""),
                            i.description.chars().take(4).collect::<String>())
                        );
                        refs_already_inserted.insert(label);
                    }
                }

                let color = colors.color_for(classes);

                let mut latex_code = format!("{{\\color{{{color}}}{}}}", validate(text));
                for label in labels {
                    latex_code = format!("\\linkdest{{{label}}}{{}}{latex_code}");
                }
                for (label, text) in refs {
                    latex_code = format!("{latex_code}\\textsuperscript{{\\hyperlink{{{label}}}{{\\color{{vforeground}}{text}}}}}");
                }
                res.push(latex_code);
            }
            Token::Newline => {
                res.push("\\\\\n".to_string())
            }
        }
    }

    res.join("")
}