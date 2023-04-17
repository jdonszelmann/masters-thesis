use crate::sourcecode::SourceCodeHash;
use crate::SourceCode;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::str::FromStr;
use itertools::Itertools;
use serde::de::Error;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Can't merge two analyses generated from different source files (source hashes of analyses don't match)")]
pub struct HashesDontMatch;

/// `start` and `len` are always in *bytes*, not in *chars*.
/// With unicode, start and len always refer to starts of code points.
#[derive(Debug, Eq, PartialEq, Hash)]
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

    fn from_parts(parts: &[(usize, usize)]) -> Option<Self> {
        match parts {
            &[] => None,
            &[(start, len), ref rest@..] => {
                let mut res = Self::new(start, len);
                res.next = Self::from_parts(rest).map(Box::new);
                Some(res)
            }
        }
    }
}

impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut res = Vec::new();
        let mut curr = self;
        let mut fmt = |part: &Self| {
            res.push(format!("{}+{}", self.start, self.len))
        };

        fmt(curr);
        while let Some(i) = &curr.next {
            fmt(i);
            curr = i;
        }

        res.join("&").serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let mut parts = Vec::new();
        for part in s.split("&") {
            let &[start, end] = part.split("+")
                .map(usize::from_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| D::Error::custom(format!("couldn't parse integer: {e}")))?
                .as_slice() else {
                return Err(D::Error::custom(format!("span part has unexpected format: {part}")));
            };

            parts.push((start, end));
        }

        Self::from_parts(&parts).ok_or_else(|| D::Error::custom(format!("zero part span: {s}")))
    }
}

type FieldRef = Span;

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Ref(FieldRef),
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

#[derive(Debug)]
pub struct Analysis {
    hash: SourceCodeHash,
    fields: Vec<(Span, Field)>,
}

impl Analysis {
    pub fn new(s: &SourceCode, fields: Vec<(Span, Field)>) -> Self {
        Self {
            hash: s.hash().clone(),
            fields,
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = &(Span, Field)> {
        self.fields.iter()
    }

    pub fn merge(self, other: Analysis) -> Result<Self, HashesDontMatch> {
        if self.hash != other.hash {
            return Err(HashesDontMatch);
        }

        Ok(Self {
            hash: self.hash,
            fields: self.fields.into_iter().chain(other.fields).collect(),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut w = Vec::new();

        let _ = writeln!(&mut w, "{}", self.hash);
        for f in &self.fields {
            serde_json::to_writer(&mut w, f).unwrap();
        }

        w
    }

    pub fn deserialize(_data: &[u8]) -> Self {
        todo!()
    }
}

impl Display for Analysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.serialize()).expect("valid utf8")
        )
    }
}
