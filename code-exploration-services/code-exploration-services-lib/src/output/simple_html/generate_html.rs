use crate::output::simple_html::sanitize_theme_name;
use crate::output::simple_html::tokenize::Token;
use crate::output::simple_html::{themes, SimpleHtmlError};
use axohtml::dom::DOMTree;
use axohtml::elements::FlowContent;
use axohtml::types::{Class, SpacedSet};
use axohtml::{html, text, unsafe_text};
use crate::textmate::theme::TextmateThemeManager;

fn generate_line_from_tokens(tokens: &[Token], line_num: usize) -> Box<dyn FlowContent<String>> {
    let mut spans = Vec::new();

    for token in tokens {
        if let Token::Token { text, classes } = token {
            let mut class = SpacedSet::new();
            for i in &classes.classes {
                let mut res = String::new();
                for i in i.split_inclusive('.') {
                    res.push_str(i);

                    class.add(Class::new(themes::sanitize_classname(
                        res.trim_matches('.'),
                    )));
                }
            }

            let res: Box<dyn FlowContent<String>> = html! {
                <span class=class data-line={line_num.to_string()}>{text!("{}", text)}</span>
            };

            spans.push(res);
        }
    }

    html! {
        <div class="code-line">{spans}</div>
    }
}

pub fn generate_html_from_tokens(
    tokens: Vec<Token>,
) -> Box<dyn FlowContent<String>> {
    let mut lines = Vec::new();
    let mut line = Vec::new();
    let mut line_num = 1;
    for i in tokens {
        if let Token::Newline = i {
            lines.push(generate_line_from_tokens(&line, line_num));
            line_num += 1;
            line = Vec::new();
        } else {
            line.push(i);
        }
    }
    lines.push(generate_line_from_tokens(&line, line_num));

    html! {
        <div class="code-view">
            {lines}
        </div>
    }
}

pub fn generate_html(
    themes: TextmateThemeManager,
    tokens: Vec<Token>,
    outline: DOMTree<String>,
    style: &str,
    script: &str,
    themes_css: String,
) -> Result<String, SimpleHtmlError> {
    let change_theme_classes = SpacedSet::try_from([
        "change-theme",
        sanitize_theme_name(&themes.iter().next().unwrap().name).as_str(),
    ])
        .unwrap();

    let lines = tokens.iter().filter(|i| matches!(i, Token::Newline)).count() + if tokens.is_empty() {
        0
    } else {
        1
    };

    let doc: DOMTree<String> = html! {
        <html>
            <head>
                <title>"CES"</title>
            </head>
            <body>
                <div id="main" class=change_theme_classes>
                    {unsafe_text!("<style>{}</style>", style)}
                    {unsafe_text!("<style>{}</style>", themes_css)}

                    <div class="theme">
                        <select id="change-theme">
                            {
                                themes.iter().map(|i| html! {
                                    <option value=sanitize_theme_name(&i.name)>{text!("{}", i.name)}</option>
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
                        <div class="nums">
                            <div class="line-numbers">
                                {(1..lines).map(|i| html!{<span>{text!("{}", i)}</span>})}
                            </div>
                        </div>
                        <div class="source">
                            {generate_html_from_tokens(tokens)}
                        </div>
                    </div>
                    <script>
                        {unsafe_text!("{}", script)}
                    </script>
                </div>
            </body>
        </html>
    };

    Ok(format!("<!DOCTYPE html>\n{}\n", doc))
}
