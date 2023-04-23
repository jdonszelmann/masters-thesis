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
}