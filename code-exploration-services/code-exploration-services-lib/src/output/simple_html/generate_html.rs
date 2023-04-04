use crate::output::simple_html::sanitize_theme_name;
use crate::output::simple_html::{themes, Classes, SimpleHtmlError};
use axohtml::dom::DOMTree;
use axohtml::elements::PhrasingContent;
use axohtml::types::{Class, SpacedSet};
use axohtml::{html, text, unsafe_text};
use textmate::theme::TextmateThemeManager;

pub fn generate_html_from_tokens(
    tokens: Vec<(String, Vec<String>)>,
) -> impl IntoIterator<Item = Box<dyn PhrasingContent<String>>> {
    tokens.into_iter().map(|(s, tags)| {
        let mut class = SpacedSet::new();
        for i in tags {
            let mut res = String::new();
            for i in i.split_inclusive('.') {
                res.push_str(i);

                class.add(Class::new(themes::sanitize_classname(
                    res.trim_matches('.'),
                )));
            }
        }

        let res: Box<dyn PhrasingContent<String>> = html! {
            <span class=class>{text!("{}", s)}</span>
        };

        res
    })
}

pub fn generate_html(
    themes: TextmateThemeManager,
    tokens: Vec<(String, Classes)>,
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
                        <pre>
                            {
                                generate_html_from_tokens(tokens)
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

    Ok(format!("<!DOCTYPE html>\n{}\n", doc))
}
