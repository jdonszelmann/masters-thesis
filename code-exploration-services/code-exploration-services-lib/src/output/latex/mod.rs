use crate::analysis::dir::{Analysis, GetAnalysisError};
use crate::output::latex::colors::{Colors, ResolveColorError};
use crate::output::scope_selector::ScopeSelectorFromStrError;
use crate::output::span_to_class;
use crate::output::theme::Theme;
use crate::output::tokenize::{index_analyses, tokenize_string, OutlineSetting, Token};
use crate::sources::dir::{ContentsError, HashError, SourceDir, SourceFile};
use crate::textmate::theme::TextmateThemeManager;
use crate::Annotater;
use convert_case::{Case, Casing};
use std::collections::HashSet;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{error, info};

mod colors;

pub struct Latex;

#[derive(Debug, Error)]
pub enum LatexError {
    #[error("get analysis for file {1:?}")]
    GetAnalysis(#[source] GetAnalysisError, PathBuf),

    #[error("contents")]
    Contents(#[from] ContentsError),

    #[error("file hash")]
    HashError(#[from] HashError),

    #[error("unknown theme: {0}")]
    UnknownTheme(String),

    #[error("parse scope selector")]
    ScopeSelectorFromTheme(#[from] ScopeSelectorFromStrError),

    #[error("resolve color in theme")]
    ResolveColor(#[from] ResolveColorError),

    #[error("had to produce code for file without name: {0:?}")]
    NoFileName(PathBuf),
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
            // theme: "3024 day".to_string(),
        }
    }
}

fn latex_safe(s: impl AsRef<str>) -> String {
    s.as_ref()
        .to_lowercase()
        .replace("-", " ")
        .replace(".", " ")
}

impl Annotater for Latex {
    type Output = Result<LatexOutput, LatexError>;
    type Params = LatexParams;

    fn annotate(&self, source: &SourceDir, a: Analysis, params: LatexParams) -> Self::Output {
        let themes = TextmateThemeManager::default();
        let Some(theme) = themes.iter().find(|i| i.name == params.theme) else {
            return Err(LatexError::UnknownTheme(params.theme.clone()));
        };
        let theme = Theme::from_textmate(theme)?;

        let mut latexes = Vec::new();
        let mut colors = Colors::new(&theme);

        let analyses = source
            .files()
            .map(|file| {
                a.analysis_for(file)
                    .map_err(|i| LatexError::GetAnalysis(i, file.path().to_path_buf()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let index = index_analyses(analyses.into_iter(), source)?;

        let mut all_labels = HashSet::new();

        for file in source.files() {
            let field_index = index.get(&file.hash()?).expect("has field index");
            let tokens = tokenize_string(
                &file.contents()?,
                0,
                &field_index,
                OutlineSetting::GenerateOutline,
            );

            for i in tokens {
                if let Token::Token { classes, .. } = i {
                    for i in &classes.reference_targets {
                        all_labels.insert(i.clone());
                    }
                }
            }
        }

        for file in source.files() {
            let field_index = index.get(&file.hash()?).expect("has field index");
            let tokens = tokenize_string(
                &file.contents()?,
                0,
                &field_index,
                OutlineSetting::GenerateOutline,
            );

            let file_name = file
                .name()
                .ok_or_else(|| LatexError::NoFileName(file.path().to_path_buf()))?;
            let name = latex_safe(&file_name);
            let parent = file
                .path()
                .parent()
                .and_then(|i| i.file_name())
                .map(|i| latex_safe(i.to_string_lossy()));

            let command_name = if let Some(parent) = parent {
                format!("code {name} ")
                // format!("code {parent} {name}")
            } else {
                format!("code {name} ")
            }
            .to_case(Case::Camel);

            colors.add_tokens(&tokens)?;
            let latex = generate_latex(file, &tokens, &colors, &all_labels)?;

            info!("Generated latex for {:?}", file.path());
            info!("Insert into latex by using \\{command_name}");

            let latex_name = validate(file_name);

            latexes.push(format!(
                r#"
\newcommand\{command_name}{{
\subsection{{{latex_name}}}
\begin{{codexcode}}
    {latex}
\end{{codexcode}}
}}
            "#
            ));
        }

        let color_definitions = colors.definitions();

        let latexes = latexes.join("\n");
        Ok(LatexOutput {
            latex_source: (
                PathBuf::from("output.tex"),
                format!(
                    r#"
{color_definitions}
{latexes}
            "#
                ),
            ),
            codex_sty: (
                PathBuf::from("codex.sty"),
                include_str!("codex.sty").to_string(),
            ),
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

fn generate_latex(
    source: SourceFile,
    tokens: &[Token],
    colors: &Colors,
    all_labels: &HashSet<String>,
) -> Result<String, LatexError> {
    let mut res = Vec::<String>::new();
    let mut labels_already_inserted = HashSet::new();

    // pre-generate annotations
    let mut annotated_tokens = tokens
        .into_iter()
        .map(|i| -> Result<_, LatexError> {
            let mut refs = Vec::new();
            let mut outline = Vec::new();

            if let Token::Token { classes, .. } = i {
                let mut refs_already_inserted = HashSet::new();
                let mut refs_from_classes = classes.references.clone();
                refs_from_classes.sort_by_key(|i| i.to.start);
                for i in refs_from_classes {
                    if i.file.is_some() {
                        error!("{:?}", i);
                    }

                    // skip self references, those only clutter the latex
                    if i.includes_self && !i.file.is_some() {
                        continue;
                    }

                    let label = span_to_class(&i.to);
                    if all_labels.contains(&label) && !refs_already_inserted.contains(&label) {
                        refs.push((
                            label.replace("-", ""),
                            i.description.chars().take(4).collect::<String>(),
                        ));
                        refs_already_inserted.insert(label);
                    }
                }

                for i in &classes.outline_targets {
                    if !i.root {
                        continue;
                    }

                    let text = validate(source.slice(&i.span)?);
                    let entry = if let Some(ref description) = i.description {
                        format!("{description} {text}")
                    } else {
                        text
                    };

                    outline.push(entry);
                }
            }

            Ok((i, refs, outline))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // deduplicate subsequent annotations
    let mut next_refs = Vec::new();
    let mut next_outline = Vec::new();
    for (_, refs, outline) in annotated_tokens.iter_mut().rev() {
        if *refs == next_refs {
            next_refs = refs.clone();
            *refs = Vec::new()
        } else {
            next_refs = refs.clone();
        }

        if *outline == next_outline {
            next_outline = outline.clone();
            *outline = Vec::new()
        } else {
            next_outline = outline.clone();
        }
    }

    // generate latex
    for (i, refs, outline) in annotated_tokens {
        match i {
            Token::Token { text, classes } => {
                if text.is_empty() {
                    continue;
                }

                let mut labels = Vec::new();
                for i in &classes.reference_targets {
                    if !labels_already_inserted.contains(&i) {
                        labels_already_inserted.insert(i);
                        labels.push(i.replace("-", ""));
                    }
                }

                let color = colors.color_for(classes);

                let mut latex_code = format!("{{\\color{{{color}}}{}}}", validate(text));
                for label in labels {
                    latex_code = format!("\\linkdest{{{label}}}{{}}{latex_code}");
                }

                let num_refs = refs.len();
                if num_refs == 1 {
                    let (label, _) = refs.first().unwrap();
                    latex_code = format!("\\hyperlink{{{label}}}{{\\color{{vforeground}}\\underline{{{latex_code}}}}}")
                } else {
                    for (idx, (label, text)) in refs.into_iter().enumerate() {
                        let separator = if idx == num_refs - 1 {
                            ""
                        } else {
                            ","
                        };

                        latex_code = format!("{latex_code}\\textsuperscript{{\\hyperlink{{{label}}}{{\\color{{vforeground}}{text}{separator}}}}}");
                    }
                }

                for entry in outline {
                    latex_code = format!("\\outlineentry{{{entry}}}{latex_code}");
                }

                res.push(latex_code);
            }
            Token::Newline => res.push("\\leavevmode \\\\\n".to_string()),
        }
    }

    Ok(res.join(""))
}
