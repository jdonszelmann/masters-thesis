use crate::dir_entry::RefConcreteDirEntry;
use crate::dir_entry::DirEntry;
use std::sync::Arc;
use std::rc::Rc;
use std::ops::Deref;

pub trait Children<'refs, 'root>
    where 'root: 'refs
{
    type Iter<'children>: Iterator<Item=RefConcreteDirEntry<'children, 'refs, 'root>>
        where Self: 'children, 'refs: 'children, 'root: 'children, 'root: 'refs, 'refs: 'root
    ;

    fn children<'children>(&'children self) -> Self::Iter<'children> where 'refs: 'children;
}

macro_rules! impl_smart_ptr {
    ($($ptr: ident),*) => {
        $(
            impl<'refs, 'root, T: DirEntry<'refs, 'root> + ?Sized + 'refs + 'root> Children<'refs, 'root> for $ptr<T>
                where 'root: 'refs, 'refs: 'root
            {
                type Iter<'children> = <T as Children<'refs, 'root>>::Iter<'children>
                    where Self: 'children, 'refs: 'children, 'root: 'children, 'root: 'refs, 'refs: 'root
                ;
                fn children<'children>(&'children self) -> Self::Iter<'children> where 'refs: 'children {
                    self.deref().children()
                }
            }
        )*
    };
}

impl_smart_ptr!(Box, Arc, Rc);
