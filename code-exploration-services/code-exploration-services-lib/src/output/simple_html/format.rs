use axohtml::elements::PhrasingContent;
use axohtml::{html, text};
use axohtml::types::{Class, SpacedSet};
use crate::output::simple_html;
use crate::output::simple_html::themes;

pub fn format_tokens(tokens: Vec<(String, Vec<String>)>) -> impl IntoIterator<Item=Box<dyn PhrasingContent<String>>> {
    tokens
        .into_iter()
        .map(|(s, tags)| {
            let mut class = SpacedSet::new();
            for i in tags {
                let mut res = String::new();
                for i in i.split_inclusive(".") {
                    res.push_str(i);

                    class.add(Class::new(themes::sanitize_classname(&res.trim_matches('.'))));
                }
            }

            let res: Box<dyn PhrasingContent<String>> =  html! {
                <span class=class>{text!("{}", s)}</span>
            };

            res
        })
}
