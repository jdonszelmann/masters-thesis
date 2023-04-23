use std::borrow::Cow;
use crate::Root;

#[derive(Clone)]
pub enum Path<'root> {
    Empty,
    Elem {
        extends: &'root Path<'root>,
        elem: Cow<'root, str>,
    }
}

impl<'root> Path<'root> {
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Path::Empty => None,
            Path::Elem { elem, .. } => Some(elem.as_ref())
        }
    }

    pub(crate) fn empty() -> Self {
        Self::Empty
    }

    pub fn add(&'root self, elem: impl Into<Cow<'root, str>>) -> Self {
        Self::Elem {
            extends: self,
            elem: elem.into(),
        }
    }
}