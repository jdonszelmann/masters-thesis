use std::fs::write;
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use tempdir::TempDir;
use thiserror::Error;
use crate::children::Children;
use crate::dir_entry::ConcreteDirEntry;
use crate::{DirEntry, SourceDir, SourceFile};

pub trait InMemoryOps<'refs, 'root>
    where 'root: 'refs, 'refs: 'root
{
    fn is_in_memory(&self) -> bool;
    fn is_on_disk(&self) -> bool;

    fn is_in_memory_recursive(&self) -> bool where Self: DirEntry<'refs, 'root> {
        self.is_in_memory() && self.children().into_iter().all(|i| i.is_in_memory())
    }
    fn is_on_disk_recursive(&self) -> bool where Self: DirEntry<'refs, 'root> {
        self.is_on_disk() && self.children().into_iter().all(|i| i.is_on_disk())
    }
}


macro_rules! impl_smart_ptr {
    ($($ptr: ident),*) => {
        $(
            impl<'refs, 'root, T: InMemoryOps<'refs, 'root> + ?Sized> InMemoryOps<'refs, 'root> for $ptr<T>
                where 'root: 'refs, 'refs: 'root
            {
                fn is_in_memory(&self) -> bool {
                    self.deref().is_in_memory()
                }

                fn is_on_disk(&self) -> bool {
                    self.deref().is_on_disk()
                }
            }
        )*
    };
}

impl_smart_ptr!(Box, Arc, Rc);
