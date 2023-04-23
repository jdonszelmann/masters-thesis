use crate::dir_entry::{ConcreteDirEntry, RefConcreteDireEntry};
use crate::DirEntry;
use std::rc::Rc;
use std::sync::Arc;
use std::ops::Deref;

pub trait Children<'refs, 'root> {
    type Iter<'children>: Iterator<Item=RefConcreteDireEntry<'children, 'refs, 'root>>
        where Self: 'children, 'refs: 'children, 'root: 'refs
    ;

    fn children<'children>(&'children self) -> Self::Iter<'children>;
}

macro_rules! impl_smart_ptr {
    ($($ptr: ident),*) => {
        $(
            impl<'root, T: DirEntry<'root> + ?Sized> Children<'root> for $ptr<T> {
                type Iter<'children> = <T as Children<'root>>::Iter<'children>
                    where Self: 'children, 'root: 'children
                ;
                fn children<'children>(&'children self) -> Self::Iter<'children> {
                    self.deref().children()
                }
            }
        )*
    };
}

// impl_smart_ptr!(Box, Arc, Rc);
