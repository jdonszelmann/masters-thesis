use serde::{Serialize, Deserialize};
use crate::sources::span::Span;

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Ref {
        description: String,
        reference: FieldRef,
    },
    SyntaxColour(String),
    Outline {
        description: Option<String>,
        parent: Option<FieldRef>,
    },
}

impl Field {
    pub fn serialize(&self, w: &mut Vec<u8>) {
        serde_json::to_writer(w, self).unwrap()
    }
}

pub type FieldRef = Span;
