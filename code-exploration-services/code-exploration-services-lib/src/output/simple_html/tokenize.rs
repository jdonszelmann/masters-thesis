use itertools::Itertools;
use crate::Analysis;
use crate::analysis::Field;
use crate::output::simple_html::{Classes, FieldIndex};

pub fn tokenize_string(s: &str, offset: usize, field_index: &FieldIndex) -> Vec<(String, Classes)> {
    let mut tokens = Vec::new();

    let mut curr = Vec::new();
    let mut active_colour_spans = Vec::<ActiveColourSpan>::new();
    let mut prev_active_colour_spans = Vec::<String>::new();

    for (index, byte) in s.bytes().enumerate() {
        let mut new_active_colour_spans = Vec::new();
        for (start, len, len_left, c)  in active_colour_spans {
            if len_left != 1 {
                new_active_colour_spans.push((start, len, len_left - 1, c))
            }
        };
        active_colour_spans = new_active_colour_spans;

        if let Some(possible_fields) = field_index.get(&(index + offset)) {
            for (span, field) in possible_fields {
                if let Field::SyntaxColour(c) = field {
                    if span.len != 0 {
                        active_colour_spans.push((span.start, span.len, span.len, c.clone()));
                    }
                }
            }
        }

        let active_names = active_colour_spans
            .iter()
            .map(|i| &i.3)
            .cloned()
            .collect_vec();
        if active_names != prev_active_colour_spans {
            tokens.push((String::from_utf8(curr).expect("valid utf8"), prev_active_colour_spans));
            curr = Vec::new();
        }

        curr.push(byte);

        prev_active_colour_spans = active_names;
    }

    let mut new_active_colour_spans = Vec::new();
    for (start, len, len_left, c)  in active_colour_spans {
        if len_left != 1 {
            new_active_colour_spans.push((start, len, len_left - 1, c))
        }
    };
    active_colour_spans = new_active_colour_spans;

    let active_names = active_colour_spans
        .iter()
        .map(|i| &i.3)
        .cloned()
        .collect_vec();
    if active_names != prev_active_colour_spans {
        tokens.push((String::from_utf8(curr).expect("valid utf8"), prev_active_colour_spans));
        curr = Vec::new();
    }

    tokens
}

pub fn index_analysis(a: &Analysis) -> FieldIndex {
    let mut fields = FieldIndex::new();
    for (s, f) in a.fields() {
        fields.entry(s.start as usize).or_insert_with(Vec::new).push((s, f));
    }

    fields
}

type ActiveColourSpan = (usize, usize, usize, String);
