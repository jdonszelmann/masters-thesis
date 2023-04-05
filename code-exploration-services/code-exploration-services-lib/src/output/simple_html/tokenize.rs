use crate::analysis::Field;
use crate::output::simple_html::FieldIndex;
use crate::Analysis;
use std::collections::HashSet;

pub struct Token {
    pub text: String,
    pub classes: Classes,
    pub id: Option<String>,
}

fn decrement_active_classes(classes: ActiveClasses) -> ActiveClasses {
    active_classes
        .into_iter()
        .filter_map(ActiveClass::decrement)
        .collect()
}

pub fn tokenize_string(s: &str, offset: usize, field_index: &FieldIndex) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut curr = Vec::new();
    let mut active_classes = ActiveClasses::new();
    let mut previous_classes = Classes::new();

    for (index, byte) in s.bytes().enumerate() {
        active_classes = decrement_active_classes(active_classes);

        if let Some(possible_fields) = field_index.get(&(index + offset)) {
            for (span, field) in possible_fields {
                if let Field::SyntaxColour(c) = field {
                    if span.len != 0 {
                        active_classes.push(ActiveClass {
                            start: span.start,
                            len: span.len,
                            len_to_go: span.len,
                            class: c.clone(),
                        });
                    }
                }
            }
        }

        let classes = Classes::from(active_classes.as_ref());
        if classes != previous_classes {
            tokens.push(Token {
                text: String::from_utf8(curr).expect("valid utf8"),
                classes: previous_classes,
                id: None,
            });
            curr = Vec::new();
        }

        curr.push(byte);

        previous_classes = classes;
    }

    active_classes = decrement_active_classes(active_classes);

    let classes = Classes::from(active_classes.as_ref());
    if classes != previous_classes {
        tokens.push(Token {
            text: String::from_utf8(curr).expect("valid utf8"),
            classes: previous_classes,
            id: None,
        });
    }

    tokens
}

pub fn index_analysis(a: &Analysis) -> FieldIndex {
    let mut fields = FieldIndex::new();
    for (s, f) in a.fields() {
        fields.entry(s.start).or_insert_with(Vec::new).push((s, f));
    }

    fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct Classes {
    pub classes: HashSet<String>,
}

impl Classes {
    pub fn new() -> Self {
        Self {
            classes: Default::default(),
        }
    }
}

impl<'a> From<&'a [ActiveClass]> for Classes {
    fn from(value: &'a [ActiveClass]) -> Self {
        Self {
            classes: value.iter().map(|i| i.class.clone()).collect(),
        }
    }
}

struct ActiveClass {
    start: usize,
    len: usize,
    len_to_go: usize,
    class: String,
}

impl ActiveClass {
    pub fn decrement(mut self) -> Option<Self> {
        if self.len_to_go > 1 {
            self.len_to_go -= 1;
            Some(self)
        } else {
            None
        }
    }
}

type ActiveClasses = Vec<ActiveClass>;
