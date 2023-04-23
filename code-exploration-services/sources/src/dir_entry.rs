use std::cell::Ref;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use crate::{Root, SourceDir, SourceFile};
use crate::in_memory::InMemoryOps;
use std::ops::Deref;
use crate::children::Children;
use crate::path::Path;

macro_rules! forward {
    (call [fn $name: ident $(<$($generic: lifetime)*>)? (&$($life:lifetime)?self $($signature: tt)*) $($ret: tt)*] with $($call: tt)*) => {
        fn $name$(<$($generic),*>)?(&$($life)?self $($signature)*) $($ret)* {
            match self {
                // ConcreteDirEntry::Ref(r) => r.$name$($call)*,
                // ConcreteDirEntry::RefcellRef(r) => r.$name$($call)*,
                ConcreteDirEntry::File(f) => f.$name$($call)*,
                ConcreteDirEntry::Dir(d) => d.$name$($call)*,
            }
        }
    };
    (fn $name: ident $(<$($generic: lifetime)*>)? (&$($life:lifetime)?self $($signature: tt)*) $($ret: tt)*) => {
        forward!(call [fn $name $(<$($generic)*>)? (&$($life)?self $($signature)*) $($ret)*] with ());
    };
}

pub enum RefConcreteDireEntry<'children, 'refs, 'root> {
    Shared(&'children ConcreteDirEntry<'refs, 'root>),
    Ref(Ref<'children, ConcreteDirEntry<'refs, 'root>>),
}

impl<'localrefs, 'refs, 'root> Deref for RefConcreteDireEntry<'localrefs, 'refs, 'root> {
    type Target = ConcreteDirEntry<'refs, 'root>;

    fn deref(&self) -> &Self::Target {
        match self {
            RefConcreteDireEntry::Shared(r) => r,
            RefConcreteDireEntry::Ref(r) => r,
        }
    }
}

pub enum ConcreteDirEntry<'refs,'root> {
    File(SourceFile<'refs, 'root>),
    Dir(SourceDir<'refs, 'root>),
}

impl<'refs, 'root> InMemoryOps<'refs, 'root> for ConcreteDirEntry<'refs, 'root> {
    forward!(fn is_in_memory(&self) -> bool);
    forward!(fn is_on_disk(&self) -> bool);
}

pub enum MergeIter<'children, 'refs, 'root>
    where 'root: 'children
{
    Dir(<SourceDir<'refs, 'root> as Children<'refs, 'root>>::Iter<'children>),
    File(<SourceFile<'refs, 'root> as Children<'refs, 'root>>::Iter<'children>),
}

impl<'children, 'refs, 'root> Iterator for MergeIter<'children, 'refs, 'root> {
    type Item = RefConcreteDireEntry<'children, 'refs, 'root>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MergeIter::Dir(d) => d.next(),
            MergeIter::File(f) => f.next(),
        }
    }
}

impl<'refs, 'root> Children<'refs, 'root> for ConcreteDirEntry<'refs, 'root> {
    type Iter<'children> = MergeIter<'children, 'refs, 'root>
        where Self: 'children, 'root: 'children
    ;

    fn children<'children>(&'children self) -> Self::Iter<'children> {
        match self {
            // ConcreteDirEntry::Ref(r) => r.children(),
            // ConcreteDirEntry::RefcellRef(r) => r.children(),
            ConcreteDirEntry::File(f) => MergeIter::File(f.children()),
            ConcreteDirEntry::Dir(d) => MergeIter::Dir(d.children())
        }
    }
}

impl<'refs, 'root> Display for ConcreteDirEntry<'refs, 'root> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl<'refs, 'root> DirEntry<'refs, 'root> for ConcreteDirEntry<'refs, 'root> {
    fn make_concrete(&'root self) -> ConcreteDirEntry<'refs, 'root> {
        todo!()
        // Self::Ref(self)
    }

    fn name<'a>(&'a self) -> &'a str where 'root: 'a {
        match self {
            ConcreteDirEntry::File(f) => f.name(),
            ConcreteDirEntry::Dir(d) => d.name(),
        }
    }

    forward!(fn path(&self) -> &Path);
    forward!(fn root(&self) -> &Root<'refs, 'root>);
    forward!(call [fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result] with (f, depth));
}


pub trait DirEntry<'refs, 'root>: InMemoryOps<'refs, 'root> + Children<'refs, 'root> + Display {
    fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result;

    fn make_concrete(&'root self) -> ConcreteDirEntry<'refs, 'root>;

    fn root(&self) -> &Root<'refs, 'root>;

    fn name<'a>(&'a self) -> &'a str where 'root: 'a {
        let path = self.path();
        let filename = path.file_name();
        let root_name = self.root().name();

        filename.unwrap_or(root_name)
    }
    fn path(&self) -> &Path;
}

macro_rules! impl_smart_ptr {
    ($($ptr: ident),*) => {
        $(
            impl<'root, T: DirEntry<'root> + ?Sized + 'root> DirEntry<'root> for $ptr<T> {
                fn make_concrete(&'root self) -> ConcreteDirEntry<'root> {
                    self.deref().make_concrete()
                }

                fn name<'a>(&'a self) -> &'a str where 'root: 'a {
                    self.deref().name()
                }
                fn path(&self) -> &Path {
                    self.deref().path()
                }
                fn root(&self) -> &Root<'root> {
                    self.deref().root()
                }
                fn pretty_print(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
                    self.deref().pretty_print(f, depth)
                }
            }
        )*
    };
}

// impl_smart_ptr!(Box, Arc, Rc);
