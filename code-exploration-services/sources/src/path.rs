use std::borrow::Cow;
use crate::Root;

#[derive(Clone)]
pub struct Path {
    elems: Vec<String>,
}

impl Path {
    pub fn file_name(&self) -> Option<&str> {
        self.elems.last().map(|i| i.as_ref())
    }

    pub(crate) fn empty() -> Self {
        Self {
            elems: vec![],
        }
    }

    pub fn add(&self, elem: &str) -> Self {
        Self {
            elems: self.elems.iter().cloned().chain(std::iter::once(elem.to_string())).collect(),
        }
    }
}