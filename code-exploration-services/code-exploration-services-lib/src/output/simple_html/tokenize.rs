use crate::analysis::field::{Field, FieldRef};
use crate::analysis::file::FileAnalysis;
use crate::output::simple_html::tokenize::Token::Newline;
use crate::output::simple_html::{outline, FieldIndex, IndexField};
use crate::sources::span::Span;
use std::collections::HashSet;
use tracing::info;

#[derive(Debug)]
pub enum Token {
    Token { text: String, classes: Classes },
    Newline,
}

fn decrement_active_classes(classes: ActiveClasses) -> ActiveClasses {
    classes
        .into_iter()
        .filter_map(ActiveClass::decrement)
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum OutlineSetting {
    GenerateOutline,
    DontGenerateOutline,
}

fn active_at_offset<'a>(
    field_index: &'a FieldIndex,
    offset: usize,
) -> impl Iterator<Item = ActiveClass> + 'a {
    field_index
        .values()
        .flatten()
        .filter_map(move |(span, field)| {
            if offset >= span.start && offset < span.start + span.len {
                if let IndexField::Field(Field::SyntaxColour(c)) = field {
                    Some(ActiveClass::colour_from_span(span, c))
                } else if let IndexField::Field(Field::Ref { description, reference }) = field {
                    Some(ActiveClass::ref_from_span(span, description, reference.clone()))
                } else if let IndexField::ReferenceTarget = field {
                    Some(ActiveClass::ref_target_from_span(span, outline::span_to_class(span)))
                } else {
                    None
                }
            } else {
                None
            }
        })
}

pub fn tokenize_string(
    s: &str,
    offset: usize,
    field_index: &FieldIndex,
    outline_setting: OutlineSetting,
) -> Vec<Token> {
    let mut tokens = Vec::new();
    // the current token
    let mut curr_token = Vec::new();
    // what classes are active for the current character
    let mut active_classes = ActiveClasses::new();
    // what classes were active for the previous character
    let mut previous_classes = Classes::new();

    // initialize with all classes which should already be active at this offset
    active_classes.extend(active_at_offset(&field_index, offset));

    for (index, byte) in s.bytes().enumerate() {
        // classes are always valid for a certain length (its span)
        // every time we go to a next character, decrement the length
        // for which each active class is still valid. Some classes
        // might become invalid and are removed from active classes
        active_classes = decrement_active_classes(active_classes);

        // find new classes that need to become active at this character
        if let Some(possible_fields) = field_index.get(&(index + offset)) {
            for (span, field) in possible_fields {
                match field {
                    IndexField::Field(Field::SyntaxColour(c)) if span.len != 0 => {
                        active_classes.push(ActiveClass::colour_from_span(span, c));
                    }
                    IndexField::Field(Field::Ref { description, reference }) => {
                        active_classes.push(ActiveClass::ref_from_span(
                            span,
                            description.clone(),
                            reference.clone(),
                        ));
                    }
                    IndexField::Field(Field::Outline { .. }) if outline_setting == OutlineSetting::GenerateOutline => {
                        active_classes
                            .push(ActiveClass::outline_target_from_span(span, outline::span_to_class(span)));
                    }
                    IndexField::ReferenceTarget  => {
                        active_classes
                            .push(ActiveClass::ref_target_from_span(span, outline::span_to_class(span)));
                    }
                    _ => {}
                }
            }
        }

        // if the classes that are active changed between this and the previous character
        // (maybe more became active, maybe fewer), then take the accumulated tokens sofar (curr),
        // make it its own token and assign it the right classes, and start a new empty token for
        // which the new classes will become valid
        let classes = Classes::from(active_classes.as_ref());
        if classes != previous_classes || byte == b'\n' {
            tokens.push(Token::Token {
                text: String::from_utf8(curr_token).expect("valid utf8"),
                classes: previous_classes,
            });
            curr_token = Vec::new();
        }

        if byte != b'\n' {
            curr_token.push(byte);
        } else {
            tokens.push(Newline);
        }
        previous_classes = classes;
    }

    tokens.push(Token::Token {
        text: String::from_utf8(curr_token).expect("valid utf8"),
        classes: previous_classes,
    });

    tokens
}

pub fn index_analysis(a: &FileAnalysis) -> FieldIndex {
    let mut fields = FieldIndex::new();
    for (s, f) in a.fields() {
        fields.entry(s.start).or_insert_with(Vec::new).push((s, IndexField::Field(f)));
        if let Field::Ref { reference, .. } = f {
            info!("reference: {reference:?}");
            fields.entry(reference.start).or_insert_with(Vec::new).push((reference, IndexField::ReferenceTarget));
        }
    }

    fields
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reference {
    pub description: String,
    pub to: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Classes {
    pub color_classes: HashSet<String>,
    pub references: Vec<Reference>,
    pub reference_targets: HashSet<String>,
    pub outline_targets: HashSet<String>,
}

impl Classes {
    pub fn new() -> Self {
        Self {
            color_classes: Default::default(),
            references: vec![],
            reference_targets: Default::default(),
            outline_targets: Default::default(),
        }
    }
}

impl<'a> From<&'a [ActiveClass]> for Classes {
    fn from(value: &'a [ActiveClass]) -> Self {
        Self {
            outline_targets: value.iter().filter_map(|i| {
                if let ActiveClass {contents: ClassContents::OutlineTarget(class), ..} = i {
                    Some(class.clone())
                } else {
                    None
                }
            }).collect(),
            color_classes: value.iter().filter_map(|i| {
                if let ActiveClass {contents: ClassContents::ColourClass(class), ..} = i {
                    Some(class.clone())
                } else {
                    None
                }
            }).collect(),
            references: value.iter().filter_map(|i| {
                if let ActiveClass {contents: ClassContents::Reference(reference), ..} = i {
                    Some(reference.clone())
                } else {
                    None
                }
            }).collect(),
            reference_targets: value.iter().filter_map(|i| {
                if let ActiveClass {contents: ClassContents::ReferenceTarget(reference), ..} = i {
                    Some(reference.clone())
                } else {
                    None
                }
            }).collect(),
        }
    }
}

enum ClassContents {
    ColourClass(String),
    Reference(Reference),
    ReferenceTarget(String),
    OutlineTarget(String),
}

struct ActiveClass {
        #[allow(dead_code)]
        start: usize,
        #[allow(dead_code)]
        len: usize,
        len_to_go: usize,
        contents: ClassContents,
}

impl ActiveClass {
    pub fn colour_from_span(span: &Span, class: impl AsRef<str>) -> Self {
        Self {
            start: span.start,
            len: span.len,
            len_to_go: span.len,
            contents: ClassContents::ColourClass(class.as_ref().to_string()),
        }
    }

    pub fn outline_target_from_span(span: &Span, target: impl AsRef<str>) -> Self {
        Self {
            start: span.start,
            len: span.len,
            len_to_go: span.len,
            contents: ClassContents::OutlineTarget(target.as_ref().to_string()),
        }
    }

    pub fn ref_from_span(span: &Span, description: impl AsRef<str>, to: FieldRef) -> Self {
        Self {
            start: span.start,
            len: span.len,
            len_to_go: span.len,
            contents: ClassContents::Reference(Reference{description: description.as_ref().to_string(), to }),
        }
    }


    pub fn ref_target_from_span(span: &Span, class: impl AsRef<str>) -> Self {
        Self {
            start: span.start,
            len: span.len,
            len_to_go: span.len,
            contents: ClassContents::ReferenceTarget(class.as_ref().to_string()),
        }
    }

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
