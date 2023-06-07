use std::collections::HashMap;
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


pub struct Colors<'b> {
    colors: HashMap<Classes, (Color, String)>,
    theme: &'b Theme<'b>,
    color_idx: usize,
}

impl<'b> Colors<'b> {
    pub fn new(theme: &'b Theme<'b>) -> Self {
        Self {
            colors: HashMap::new(),
            theme,
            color_idx: 0,
        }
    }

    pub fn add_tokens(&mut self, tokens: &[Token]) -> Result<(), ResolveColorError> {
        for i in tokens {
            if let Token::Token {classes, ..} = i {
                if !self.colors.contains_key(&classes) {
                    self.colors.insert(classes.clone(), (
                        resolve_color(&classes.color_classes, self.theme)?,
                        format!("color{}", self.color_idx))
                    );
                    self.color_idx += 1;
                }
            }
        }

        Ok(())
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

