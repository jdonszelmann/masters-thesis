use serde::{Deserialize, Serialize};
use crate::sourcecode::SourceCodeHash;
use std::io::Write;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Can't merge two analyses generated from different source files (source hashes of analyses don't match)")]
pub struct HashesDontMatch;

#[derive(Serialize, Deserialize)]
pub struct Span {
    start: usize,
    len: usize,
    next: Option<Box<Span>>,
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

#[derive(Serialize, Deserialize)]
pub enum Field {
    Ref(Span),
    Class(),
}

impl Field {
    pub fn serialize(&self, w: &mut Vec<u8>) {
        serde_json::to_writer(w, self)
            .unwrap()
    }
}

pub struct Analysis {
    hash: SourceCodeHash,
    fields: Vec<(Span, Field)>
}

impl Analysis {
    pub fn merge(self, other: Analysis) -> Result<Self, HashesDontMatch> {
        if self.hash != other.hash {
            return Err(HashesDontMatch)
        }

        Ok(Self {
            hash: self.hash,
            fields: self.fields.into_iter().chain(other.fields).collect(),
        })
    }


    fn serialize(&self) -> Vec<u8> {
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

    fn deserialize(_data: &[u8]) -> Self {
        todo!()
    }
}

