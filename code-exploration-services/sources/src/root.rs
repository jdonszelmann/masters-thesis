use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::fs::write;
use std::io;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::path::PathBuf;
use bumpalo::Bump;
use once_cell::sync::OnceCell;
use tempdir::TempDir;
use crate::dir::SourceDir;
use crate::{DirEntry, SourceFile};
use crate::in_memory::InMemoryOps;
use crate::path::Path;
use thiserror::Error;
use crate::dir_entry::ConcreteDirEntry;

#[derive()]
pub struct Root<'refs, 'root> {
    dir: ManuallyDrop<OnceCell<&'root SourceDir<'refs, 'root>>>,
    /// Some when on disk, None when in memory
    path: Option<PathBuf>,
    name: String,
    pub(crate) arena: ManuallyDrop<&'static Bump>,
}

#[derive(Debug, Error)]
pub enum MakeOnDiskError {
    /// returned when [`MakeOnDiskStrategy`] is Temp, and creating
    /// the temp directory fails
    #[error("create temp directory")]
    CreateTempDir(#[source] io::Error)
}

#[derive(Debug, Error)]
pub enum MakeInMemoryError {}

#[derive(Clone)]
pub enum MakeOnDiskStrategy {
    Path(PathBuf),
    Temp,
}

impl<'refs, 'root> Display for Root<'refs, 'root> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().pretty_print(f, 0)
    }
}

impl<'refs, 'root> Root<'refs, 'root> {
    pub fn new(name: impl Into<Cow<'root, str>>) -> &'root Self {
        let arena = Box::leak(Box::new(Bump::new()));

        let root = arena.alloc(Root {
            dir: ManuallyDrop::default(),
            path: None,
            name: name.into().to_string(),
            arena: ManuallyDrop::new(arena),
        });

        root.dir.get_or_init(|| arena.alloc(SourceDir::InMemory {
            root,
            path: root.arena.alloc(Path::empty()),
            entries: RefCell::new(vec![]),
        }));

        root
    }

    pub fn make_in_memory(&self) {}

    pub fn make_on_disk(&mut self, strategy: MakeOnDiskStrategy) -> Result<(), MakeOnDiskError> where Self: DirEntry<'refs, 'root> {
        // no need to do anything
        if self.is_on_disk_recursive() {
            return Ok(())
        }

        let (path, cleanup) = match strategy {
            MakeOnDiskStrategy::Path(p) => {
                (p, None)
            }
            MakeOnDiskStrategy::Temp => {
                let dir = TempDir::new(".sources")
                    .map_err(MakeOnDiskError::CreateTempDir)?;
                (
                    dir.path().to_path_buf(),
                    Some(
                        self.arena().alloc(move || {
                            drop(dir);
                        })
                    )
                )
            }
        };

        match self.make_concrete() {
            ConcreteDirEntry::File(SourceFile::InMemory {contents, ..}) => {
                write(path, contents.as_bytes()).unwrap();
            }
            ConcreteDirEntry::Dir(SourceDir::InMemory {..}) => {}
            _ => unreachable!("must be in memory because of recursive check above")
        }

        Ok(())
    }
}

impl<'refs, 'root> Root<'refs, 'root> {
    pub fn name(&self) -> &str {
        self.name.deref()
    }
}

impl<'refs, 'root> Drop for Root<'refs, 'root> {
    fn drop(&mut self) {
        // Safety: in drop, we can safely drop this. Nothing will try to use dir anymore
        unsafe { ManuallyDrop::drop(&mut self.dir); }
        // Safety: the only thing allocated in arena, was the dir. That was just dropped
        // so nothing actually needs the backing storage anymore. We can safely take it.
        let arena = unsafe { ManuallyDrop::take(&mut self.arena) };

        // deallocate the arena
        // Safety: this used to a box which we then leaked
        let _ = unsafe { Box::from_raw(arena as *const _ as *mut Bump) };
    }
}

impl<'refs, 'root> Deref for Root<'refs, 'root> {
    type Target = &'root SourceDir<'refs, 'root>;

    fn deref(&self) -> &Self::Target {
        self.dir.get().unwrap()
    }
}