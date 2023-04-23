use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::iter::Empty;
use std::rc::Rc;
// use crate::children::Children;
use crate::dir_entry::{ConcreteDirEntry, RefConcreteDireEntry};
use crate::{DirEntry, Root};
use crate::children::Children;
use crate::in_memory::InMemoryOps;
use crate::path::Path;

pub enum SourceFile<'refs, 'root> {
    Ref(&'refs SourceFile<'refs, 'root>),
    InMemory {
        root: &'root Root<'refs, 'root>,
        name: String,
    },
    OnDisk {
        root: &'root Root<'refs, 'root>,
        path: Path,
    },
}

impl<'refs, 'root> Children<'refs, 'root> for SourceFile<'refs, 'root> {
    type Iter<'children> = Empty<RefConcreteDireEntry<'children, 'refs, 'root>>
        where Self: 'children, 'root: 'children
    ;

    fn children<'children>(&'children self) -> Self::Iter<'children> {
        std::iter::empty()
    }

}

impl<'refs, 'root> Display for SourceFile<'refs, 'root> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl<'refs, 'root> DirEntry<'refs, 'root> for SourceFile<'refs, 'root> {
    fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, "{:level$}", "", level = depth * 4)?;
        write!(f, "{}", self.name())?;
        if self.is_in_memory() {
            write!(f, "(in memory)")?;
        }
        writeln!(f)?;
        // for i in self.children() {
        //     i.pretty_print(f, depth + 1);
        // }
        Ok(())
    }

    fn make_concrete(&'root self) -> ConcreteDirEntry<'refs, 'root> {
        // ConcreteDirEntry::File(Self::Ref(self))
        todo!()
    }

    fn root(&self) -> &'root Root<'refs, 'root> {
        match self {
            // SourceFile::Ref(r) => r.root(),
            SourceFile::InMemory { root, .. } => root,
            SourceFile::OnDisk { root, .. } => root,
        }
    }

    fn name<'a>(&'a self) -> &'a str where 'root: 'a {
        todo!()
    }

    fn path(&self) -> &Path {
        todo!()
    }
}
impl<'refs, 'root> InMemoryOps<'refs, 'root> for SourceFile<'refs, 'root> {
    fn is_in_memory(&self) -> bool {
        matches!(self, Self::InMemory {..})
    }

    fn is_on_disk(&self) -> bool {
        matches!(self, Self::OnDisk {..})
    }
}

impl SourceFile<'_, '_> {

}