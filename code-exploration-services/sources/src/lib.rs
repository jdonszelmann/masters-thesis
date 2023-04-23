mod dir;
mod dir_entry;
mod file;
mod span;
mod root;
mod path;

mod in_memory;
mod children;

pub use root::Root;
pub use span::SourceSpan;
pub use file::SourceFile;
pub use dir_entry::DirEntry;
pub use dir::SourceDir;

macro_rules! root {
    (
        $parent:ident@$name: literal: [$($dir:tt)*], $($rest: tt)*
    ) => {
        let res = $parent.create_dir($name);
        root!(res@$($dir)*);
        root!($parent@$($rest)*)
    };
    (
        $parent:ident@$name: ident: [$($dir:tt)*], $($rest: tt)*
    ) => {
        let res = $parent.create_dir(stringify!($name));
        root!(res@$($dir)*);
        root!($parent@$($rest)*)
    };
    (
        $parent:ident@#$name: ident: [$($dir:tt)*], $($rest: tt)*
    ) => {
        let res = $parent.create_dir(stringify!($name));
        root!(res@$($dir)*);
        root!($parent@$($rest)*)
    };
    (
        $parent:ident@$name: literal: $contents: literal, $($rest: tt)*
    ) => {
        $parent.create_file(stringify!($name), $contents);
        root!($parent@$($rest)*)
    };
    (
        $parent:ident@$name: ident: $contents: literal, $($rest: tt)*
    ) => {
        a
    };
    (
        $parent:ident@#$name: ident: $contents: literal, $($rest: tt)*
    ) => {
        b
    };

    (
        $parent:ident@$name: literal: #$contents: ident, $($rest: tt)*
    ) => {
        let res = $parent.create_file(stringify!($name), $contents);
        root!($parent@$($rest)*);
    };
    (
        $parent:ident@$name: ident: #$contents: ident, $($rest: tt)*
    ) => {
        c
    };
    (
        $parent:ident@#$name: ident: #$contents: ident, $($rest: tt)*
    ) => {
        d
    };

    (
        $name: literal: [$($tt: tt)*]
    ) => {
        {
            let res = $crate::Root::new($name);
            root!(res@$($tt)*);
            res
        }
    };
    (
        $name: ident: [$($tt: tt)*] $(,)?
    ) => {
        {
            let res = $crate::Root::new(stringify!($name));
            root!(res @ $($tt)*);
            res
        }
    };
    (
        #$name: ident: [$($tt: tt)*] $(,)?
    ) => {
        {
            let res = $crate::Root::new($name);
            root!(res @ $($tt)*);
            res
        }
    };
    () => {};
    ($parent: ident@) => { };
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        let b_contents = "Lorem Ipsum";

        let r = root!(
            test: [
                "a.rs": "test",
                "b.rs": "yeet",
                yeet: [
                    "x.rs": #b_contents,
                ],
            ],
        );

        println!("{}", r);
    }
}

