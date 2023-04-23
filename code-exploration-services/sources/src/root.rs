use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::path::PathBuf;
use bumpalo::Bump;
use once_cell::sync::OnceCell;
use crate::dir::SourceDir;
use crate::DirEntry;
use crate::path::Path;

#[derive()]
pub struct Root<'refs, 'root> {
    dir: ManuallyDrop<OnceCell<&'root SourceDir<'refs, 'root>>>,
    /// Some when on disk, None when in memory
    path: Option<PathBuf>,
    name: String,
    arena: ManuallyDrop<&'static Bump>,
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
            path: Path::empty(),
            entries: RefCell::new(vec![]),
        }));

        root
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