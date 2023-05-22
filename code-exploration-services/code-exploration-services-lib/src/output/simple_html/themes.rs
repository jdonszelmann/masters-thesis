use crate::output::simple_html::SimpleHtmlError;
use std::iter;
use std::str::FromStr;
use thiserror::Error;
use crate::textmate::theme::{FontStyle, Settings, SettingsItem, TextmateTheme, TextmateThemeManager};

pub fn generate_theme_styles(themes: &TextmateThemeManager) -> Result<String, SimpleHtmlError> {
    let mut res = String::new();

    for theme in themes.iter() {
        res.push_str(generate_theme_style(theme)?.as_str())
    }

    Ok(res)
}

fn generate_theme_style(theme: &TextmateTheme) -> Result<String, SimpleHtmlError> {
    let mut res = String::new();
    let mut global_settings = Settings::default();

    for settings_item in &theme.settings {
        if let SettingsItem::Settings { settings: s } = settings_item {
            global_settings = s.clone();
        }
    }

    res.push_str(
        format!(
            "
.{} {{
    color: {};
}}
    ",
            sanitize_theme_name(&theme.name),
            global_settings.foreground
        )
        .as_str(),
    );

    res.push_str(
        format!(
            "
.{} .highlighted {{
    background: {} !important;
}}
    ",
            sanitize_theme_name(&theme.name),
            global_settings.selection
        )
            .as_str(),
    );

    res.push_str(
        format!(
            "
.{} {{
    background: {};
}}
    ",
            sanitize_theme_name(&theme.name),
            global_settings.background
        )
        .as_str(),
    );

    for settings_item in &theme.settings {
        if let SettingsItem::Style {
            scope, settings, ..
        } = settings_item
        {
            let s = ScopeSelector::from_str(scope)?;
            for class in s.classes() {
                let underline = if settings
                    .font_style
                    .as_ref()
                    .map(|i| i == &FontStyle::Underline)
                    .unwrap_or(false)
                {
                    "text-decoration: underline;"
                } else {
                    ""
                };
                let bold = if settings
                    .font_style
                    .as_ref()
                    .map(|i| i == &FontStyle::Bold)
                    .unwrap_or(false)
                {
                    "font-weight: bold;"
                } else {
                    ""
                };
                let italics = if settings
                    .font_style
                    .as_ref()
                    .map(|i| i == &FontStyle::Italic)
                    .unwrap_or(false)
                {
                    "font-style: italic;"
                } else {
                    ""
                };

                res.push_str(
                    format!(
                        "
.{} {} {{
    color: {};
    background: {};
    {}
    {}
    {}
}}
                ",
                        sanitize_theme_name(&theme.name),
                        class,
                        settings.foreground.unwrap_or(global_settings.foreground),
                        settings.background.unwrap_or(global_settings.background),
                        underline,
                        italics,
                        bold,
                    )
                    .as_str(),
                );
            }
        }
    }

    Ok(res)
}

pub fn sanitize_classname(inp: &str) -> String {
    inp.trim().to_lowercase().replace([' ', '.'], "-")
}

pub fn sanitize_theme_name(inp: &str) -> String {
    let res = inp.trim().to_lowercase().replace([' ', '.'], "-");

    format!("theme-{}", res)
}

#[derive(Debug, Error)]
pub enum ScopeSelectorFromStrError {
    #[error("{0} doesn't match any scope")]
    Empty(String),

    #[error("while parsing {0}: {1}")]
    Inside(String, Box<ScopeSelectorFromStrError>),
}

#[cfg(test)]
mod tests {
    use crate::output::simple_html::themes::ScopeSelector;
    use std::str::FromStr;

    #[test]
    fn test_classes() {
        macro_rules! classes_test {
            ($inp: literal, $($res: literal),* $(,)?) => {
                assert_eq!(
                    ScopeSelector::from_str($inp).unwrap().classes().collect::<Vec<_>>(),
                    vec![$($res),*],
                )
            };
        }

        classes_test!("a.b", ".a-b");
        classes_test!("a.b a.b", ".a-b.a-b");
        classes_test!("a.b,a.c", ".a-b", ".a-c");
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
    pub fn classes<'a>(&'a self) -> Box<dyn Iterator<Item = String> + 'a> {
        match self {
            ScopeSelector::Selector(s) => Box::new(iter::once(format!(".{s}"))),
            ScopeSelector::Or(items) => Box::new(items.iter().flat_map(|i| i.classes())),
            ScopeSelector::Inside(a, b) => Box::new(b.classes().map(move |i| format!(".{a}{i}"))),
        }
    }

    fn root_from_str(s: &str) -> Result<Self, ScopeSelectorFromStrError> {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.is_empty() {
            Err(ScopeSelectorFromStrError::Empty(s.to_string()))
        } else if parts.len() == 1 {
            Self::inside_from_str(parts[0])
        } else {
            let mut res = Vec::new();
            for i in parts {
                res.push(ScopeSelector::from_str(i).map_err(|e| {
                    ScopeSelectorFromStrError::Inside(s.replace('.', "-"), Box::new(e))
                })?);
            }

            Ok(Self::Or(res))
        }
    }

    fn inside_from_str(s: &str) -> Result<Self, ScopeSelectorFromStrError> {
        let parts = s
            .splitn(2, ' ')
            .filter(|i| !i.trim().is_empty())
            .collect::<Vec<_>>();
        if parts.is_empty() {
            Err(ScopeSelectorFromStrError::Empty(s.to_string()))
        } else if parts.len() == 1 {
            Ok(Self::Selector(parts[0].replace('.', "-")))
        } else {
            Ok(Self::Inside(
                parts[0].trim().replace('.', "-"),
                Box::new(
                    Self::inside_from_str(parts[1].trim()).map_err(|e| {
                        ScopeSelectorFromStrError::Inside(s.to_string(), Box::new(e))
                    })?,
                ),
            ))
        }
    }
}
