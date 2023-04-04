use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::sourcecode::SourceCodeHash;
use std::io::Write;
use thiserror::Error;
use crate::SourceCode;

#[derive(Debug, Error)]
#[error("Can't merge two analyses generated from different source files (source hashes of analyses don't match)")]
pub struct HashesDontMatch;

/// `start` and `len` are always in *bytes*, not in *chars*.
/// With unicode, start and len always refer to starts of code points.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub struct Span {
    pub start: usize,
    pub len: usize,
    pub next: Option<Box<Span>>,
}

impl Span {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
            next: None,
        }
    }

    pub fn midpoint(&self) -> usize {
        self.start + self.len / 2
    }

    pub fn from_start_end(start: usize, end: usize) -> Self {
        assert!(end >= start, "end should be after start");
        Self::new(start, end - start)
    }
}

impl Span {
    pub fn serialize(&self, w: &mut Vec<u8>) {
        let _ = write!(
            w,
            "{}+{}",
            self.start,
            self.len
        );
        if let Some(ref i) = self.next {
            let _ = write!(w, "&");
            Span::serialize(i, w);
        }
    }
}

type FieldRef = Span;

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Ref(FieldRef),
    SyntaxColour(String),
    Outline {
        description: Option<String>,
        parent: Option<FieldRef>
    }
}

impl Field {
    pub fn serialize(&self, w: &mut Vec<u8>) {
        serde_json::to_writer(w, self)
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Analysis {
    hash: SourceCodeHash,
    fields: Vec<(Span, Field)>
}

impl Analysis {
    pub fn new(s: &SourceCode, fields: Vec<(Span, Field)>) -> Self {
        Self {
            hash: s.hash().clone(),
            fields,
        }
    }

    pub fn fields(&self) -> impl Iterator<Item=&(Span, Field)> {
        self.fields.iter()
    }

    pub fn merge(self, other: Analysis) -> Result<Self, HashesDontMatch> {
        if self.hash != other.hash {
            return Err(HashesDontMatch)
        }

        Ok(Self {
            hash: self.hash,
            fields: self.fields.into_iter().chain(other.fields).collect(),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut w = Vec::new();

        let _ = writeln!(&mut w, "{}", self.hash);
        for (span, field) in &self.fields {
            span.serialize(&mut w);
            let _ = write!(&mut w, ";");
            field.serialize(&mut w);
            let _ = writeln!(&mut w);
        }

        w
    }

    pub fn deserialize(_data: &[u8]) -> Self {
        todo!()
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.serialize()).expect("valid utf8"))
    }
}
