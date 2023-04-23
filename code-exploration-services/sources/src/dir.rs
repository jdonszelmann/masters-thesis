use std::alloc::alloc;
use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::rc::Rc;
// use crate::children::Children;
use crate::dir_entry::{ConcreteDirEntry, RefConcreteDireEntry};
use crate::{DirEntry, Root, SourceFile};
use crate::children::Children;
use crate::in_memory::InMemoryOps;
use crate::path::Path;

pub enum SourceDir<'refs, 'root> {
    Ref(&'refs SourceDir<'refs, 'root>),
    InMemory {
        root: &'root Root<'refs, 'root>,
        path: Path,
        entries: RefCell<Vec<ConcreteDirEntry<'refs, 'root>>>,
    },
    OnDisk {
        root: &'root Root<'refs, 'root>,
        path: Path,
    }
}

impl<'refs, 'root> SourceDir<'refs, 'root> {
    pub fn create_dir(&self, name: &str) -> &'root SourceDir<'refs, 'root> {
        todo!()
    }

    pub fn create_file(&self, name: &str, contents: impl AsRef<str>) -> &'root SourceFile<'refs, 'root> {
        todo!()
    }
}

impl<'refs, 'root> Display for SourceDir<'refs, 'root> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl<'refs, 'root> DirEntry<'refs, 'root> for SourceDir<'refs, 'root> {
    fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, "{:level$}", "", level = depth * 4)?;
        write!(f, "{}", self.name())?;
        if self.is_in_memory() {
            write!(f, "(in memory)")?;
        }
        writeln!(f)?;
        for i in self.children() {
            i.pretty_print(f, depth + 1)?;
        }
        Ok(())
    }

    fn make_concrete(&self) -> ConcreteDirEntry<'refs, 'root> {
        ConcreteDirEntry::Dir(Self::Ref(self))
    }

    fn root(&self) -> &Root<'refs, 'root> {
        match self {
            SourceDir::InMemory { root, .. } => root,
            SourceDir::OnDisk { root, .. } => root,
            SourceDir::Ref(r) => r.root(),
        }
    }

    fn path(&self) -> &Path {
        match self {
            // SourceDir::Ref(r) => r.path(),
            SourceDir::InMemory { path, ..  } => path,
            SourceDir::OnDisk { path, .. } => path
        }
    }
}

impl<'refs, 'root> InMemoryOps<'refs, 'root> for SourceDir<'refs, 'root> {
    fn is_in_memory(&self) -> bool {
        matches!(self, Self::InMemory {..})
    }

    fn is_on_disk(&self) -> bool {
        matches!(self, Self::OnDisk {..})
    }
}

impl<'refs, 'root> Children<'refs, 'root> for SourceDir<'refs, 'root> {
    type Iter<'children> = Iter<'children, 'refs, 'root>
        where Self: 'children, 'root: 'children
    ;

    fn children<'children>(&'children self) -> Self::Iter<'children> {
        match self {
            SourceDir::InMemory { entries, .. } => {
                Iter {
                    inner: Some(Ref::map(entries.borrow(), |v| &v[..])),
                }
            }
            SourceDir::OnDisk { .. } => {
                todo!()
            }
            // SourceDir::Ref(a) => a.children(),
        }
    }
}

pub struct Iter<'children, 'refs, 'root> {
    inner: Option<Ref<'children, [ConcreteDirEntry<'refs, 'root>]>>,
}
impl<'children, 'refs, 'root> Iterator for Iter<'refs, 'children, 'root>
    where 'refs: 'children
{
    type Item = RefConcreteDireEntry<'children, 'refs, 'root>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.take() {
            Some(borrow) => match *borrow {
                [] => None,
                [_, ..] => {
                    let (head, tail) = Ref::map_split(borrow, |slice| {
                        (&slice[0], &slice[1..])
                    });
                    self.inner.replace(tail);
                    Some(RefConcreteDireEntry::Ref(head))
                }
            },
            None => None,
        }
    }
}