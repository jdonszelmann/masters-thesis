use crate::analysis::field::Field;
use crate::analysis::file::FileAnalysis;
use crate::output::simple_html::generate_html::generate_html_from_tokens;
use crate::output::simple_html::generate_html::GenerateForOutlineStatus::GenerateForOutline;
use crate::output::simple_html::tokenize::OutlineSetting::DontGenerateOutline;
use crate::output::simple_html::{tokenize, FieldIndex, SimpleHtmlError};
use crate::sources::dir::SourceFile;
use crate::sources::span::Span;
use axohtml::dom::DOMTree;
use axohtml::{html, text, unsafe_text};
use std::collections::VecDeque;

#[derive(Debug)]
struct OutlineItem<'a> {
    span: &'a Span,
    children: Vec<OutlineItem<'a>>,
    description: &'a Option<String>,
}

fn insert<'a>(
    outline: &mut Vec<OutlineItem<'a>>,
    span: &'a Span,
    parent: &'a Span,
    description: &'a Option<String>,
) -> bool {
    for i in outline {
        if i.span == parent {
            i.children.push(OutlineItem {
                span,
                description,
                children: vec![],
            });
            return true;
        }

        if insert(&mut i.children, span, parent, description) {
            return true;
        }
    }

    false
}

fn generate_outline_html(
    outline: &[OutlineItem],
    parent: &OutlineItem,
    index: &FieldIndex,
    source: SourceFile,
) -> Result<DOMTree<String>, SimpleHtmlError> {
    let contents = outline
        .iter()
        .map(|i| -> Result<_, SimpleHtmlError> {
            let elem = generate_outline_html(&i.children, i, index, source)?.to_string();
            Ok(unsafe_text!("{}", elem))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let source_text = source.slice(parent.span)?;
    let tokens =
        tokenize::tokenize_string(&source_text, parent.span.start, index, DontGenerateOutline);

    let heading = if let Some(description) = parent.description {
        text!("{}: ", description)
    } else {
        text!("")
    };
    let goto_class = span_to_class(parent.span);

    let doc: DOMTree<String> = html! {
        <div class="outline-item">
            <div class="outline-header" data-goto-class=goto_class>
                <span>{heading}</span>
                {generate_html_from_tokens(tokens, GenerateForOutline, source)}
            </div>
            {
                contents
            }
        </div>
    };

    Ok(doc)
}

fn sort_outline(outline: &mut Vec<OutlineItem>) {
    outline.sort_by_key(|i| i.span.start);

    for i in outline {
        sort_outline(&mut i.children);
    }
}

pub fn generate_outline(
    analysis: &FileAnalysis,
    index: &FieldIndex,
    source: SourceFile,
) -> Result<DOMTree<String>, SimpleHtmlError> {
    let mut todo = VecDeque::new();
    let mut outline = Vec::new();

    for (span, field) in analysis.fields() {
        if let Field::Outline {
            parent,
            description,
        } = field
        {
            if let Some(parent) = parent {
                todo.push_back((span, parent, description));
            } else {
                outline.push(OutlineItem {
                    span,
                    children: vec![],
                    description,
                });
            }
        }
    }

    while let Some((span, parent, description)) = todo.pop_back() {
        if !insert(&mut outline, span, parent, description) {
            todo.push_back((span, parent, description));
        }
    }

    sort_outline(&mut outline);

    let contents = outline
        .iter()
        .map(|i| -> Result<_, SimpleHtmlError> {
            let elem = generate_outline_html(&i.children, i, index, source)?.to_string();
            Ok(unsafe_text!("{}", elem))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let doc: DOMTree<String> = html! {
        <div class="outline-root">
            {
                contents
            }
        </div>
    };

    Ok(doc)
}

pub fn span_to_class(span: &Span) -> String {
    fn span_to_class_helper(span: &Span) -> String {
        if let Some(ref i) = span.next {
            format!("{}-{}-{}", span.start, span.len, span_to_class_helper(i))
        } else {
            format!("{}-{}", span.start, span.len)
        }
    }

    format!("goto-{}", span_to_class_helper(span))
}
