use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use crate::children::Children;

pub enum MakeOnDiskStrategy {
    Path(PathBuf),
    Temp,
}

pub trait InMemoryOps<'refs, 'root>
    where 'root: 'refs
{
    fn is_in_memory(&self) -> bool;
    fn is_on_disk(&self) -> bool;

    fn is_in_memory_recursive(&'root self) -> bool where Self: Children<'refs, 'root> {
        // self.is_in_memory() && self.children().into_iter().all(|i| i.is_in_memory())
        todo!()
    }
    fn is_on_disk_recursive(&'root self) -> bool where Self: Children<'refs, 'root> {
        // self.is_on_disk() && self.children().into_iter().all(|i| i.is_on_disk())
        todo!()
    }

    fn make_on_disk(&self, strategy: MakeOnDiskStrategy) {
        todo!()
    }
}

macro_rules! impl_smart_ptr {
    ($($ptr: ident),*) => {
        $(
            impl<'refs, 'root, T: InMemoryOps<'refs, 'root> + ?Sized> InMemoryOps<'refs, 'root> for $ptr<T>
                where 'root: 'refs
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

// fn is_in_memory(&self) -> bool {
//     matches!(self, Self::InMemory {..})
// }
//
// fn is_on_disk(&self) -> bool {
//     matches!(self, Self::OnDisk {..})
// }
//
// fn is_in_memory_recursive(&self) -> bool {
//     self.is_in_memory() && self.children().all(|i| i.)
// }
