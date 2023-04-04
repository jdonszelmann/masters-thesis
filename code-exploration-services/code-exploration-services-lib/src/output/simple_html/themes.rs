use textmate::theme::{FontStyle, Settings, SettingsItem, TextmateTheme, TextmateThemeManager};
use thiserror::Error;
use std::str::FromStr;
use crate::output::simple_html::{ScopeSelector, SimpleHtmlError};

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
        match settings_item {
            SettingsItem::Settings { settings: s } => {
                global_settings = s.clone();
            }
            _ => {}
        }
    }

    res.push_str(format!(
        "
.{} {{
    color: {};
}}
    ",
        sanitize_theme_name(&theme.name),
        global_settings.foreground
    ).as_str());

    res.push_str(format!(
        "
.{} {{
    background: {};
}}
    ",
        sanitize_theme_name(&theme.name),
        global_settings.background
    ).as_str());

    for settings_item in &theme.settings {
        match settings_item {
            SettingsItem::Style { scope, settings , ..} => {
                let s = ScopeSelector::from_str(scope)?;
                for class in s.classes() {
                    // println!("{}", class);

                    let underline = if settings.font_style.as_ref().map(|i| i == &FontStyle::Underline).unwrap_or(false) {
                        "text-decoration: underline;"
                    } else {
                        ""
                    };
                    let bold = if settings.font_style.as_ref().map(|i| i == &FontStyle::Bold).unwrap_or(false) {
                        "font-weight: bold;"
                    } else {
                        ""
                    };
                    let italics = if settings.font_style.as_ref().map(|i| i == &FontStyle::Italic).unwrap_or(false) {
                        "font-style: italic;"
                    } else {
                        ""
                    };


                    res.push_str(format!(
                        "
.{} span{} {{
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
                    ).as_str());
                }
            }
            _ => {}
        }
    }

    Ok(res)
}

pub fn sanitize_classname(inp: &str) -> String {
    inp
        .trim()
        .to_lowercase()
        .replace(" ", "-")
        .replace(".", "-")
}


pub fn sanitize_theme_name(inp: &str) -> String {
    let res = inp
        .trim()
        .to_lowercase()
        .replace(" ", "-")
        .replace(".", "-");

    format!("theme-{}", res)
}

#[derive(Debug, Error)]
pub enum ScopeSelectorFromStrError {
    #[error("{0} doesn't match any scope")]
    Empty(String),

    #[error("while parsing {0}: {1}")]
    Inside(String, Box<ScopeSelectorFromStrError>)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::output::simple_html::ScopeSelector;

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
