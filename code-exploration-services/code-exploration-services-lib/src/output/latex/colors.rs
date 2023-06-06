use std::collections::HashMap;
use itertools::Itertools;
use thiserror::Error;
use crate::output::theme::Theme;
use crate::output::tokenize::{Classes, ColorClasses, Token};
use crate::textmate::theme::Color;

#[derive(Debug, Error)]
pub enum ResolveColorError {
    #[error("theme does not define a colour for itme with classes {0:?}, nor a default colour")]
    NotInTheme(ColorClasses)
}

fn resolve_color(classes: &ColorClasses, theme: &Theme) -> Result<Color, ResolveColorError> {
    let style = theme.style_for_classes(classes);

    style.foreground
        .or(theme.global.foreground)
        .ok_or(ResolveColorError::NotInTheme(classes.clone()))
}


pub struct Colors<'a, 'b> {
    colors: HashMap<&'a Classes, (Color, String)>,
    theme: &'b Theme<'b>
}

impl<'a, 'b> Colors<'a, 'b> {
    pub fn from_tokens(t: &'a [Token], theme: &'b Theme<'b>) -> Result<Self, ResolveColorError> {
        let mut colors: HashMap<&'a Classes, (Color, String)> = HashMap::new();
        let mut color_idx = 0;

        for i in t {
            if let Token::Token {classes, ..} = i {
                if !colors.contains_key(&classes) {
                    colors.insert(classes, (
                        resolve_color(&classes.color_classes, theme)?,
                        format!("color{color_idx}"))
                    );
                    color_idx += 1;
                }
            }
        }

        Ok(Self {
            colors,
            theme,
        })
    }

    pub fn color_for(&self, classes: &Classes) -> &str {
        &self.colors.get(classes)
            .expect("what? we indexed these tokens in from_tokens")
            .1
    }

    pub fn definitions(&self) -> String {
        let mut res = Vec::new();
        for (color, name) in self.colors.values() {
            let Color(r, g, b, _) = color;
            res.push(format!("\\definecolor{{{name}}}{{RGB}}{{{r},{g},{b}}}"))
        }

        let Color(r, g, b, _) = self.theme.global.background.expect("global background color");
        res.push(format!("\\definecolor{{vbackground}}{{RGB}}{{{r},{g},{b}}}"));
        let Color(r, g, b, _) = self.theme.global.foreground.expect("global background color");
        res.push(format!("\\definecolor{{vforeground}}{{RGB}}{{{r},{g},{b}}}"));


        res.join("\n")
    }
}

