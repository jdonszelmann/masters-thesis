use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// `start` and `len` are always in *bytes*, not in *chars*.
/// With unicode, start and len always refer to starts of code points.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    pub fn includes(&self, other: &Self) -> bool {
        self.start <= other.start && self.end() >= other.end()
    }

    pub fn end(&self) -> usize {
        self.start + self.len
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
            &[(start, len), ref rest @ ..] => {
                let mut res = Self::new(start, len);
                res.next = Self::from_parts(rest).map(Box::new);
                Some(res)
            }
        }
    }
}

impl Serialize for Span {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = Vec::new();
        let mut curr = self;
        let mut fmt = |_part: &Self| res.push(format!("{}+{}", self.start, self.len));

        fmt(curr);
        while let Some(i) = &curr.next {
            fmt(i);
            curr = i;
        }

        res.join("&").serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Span {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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
