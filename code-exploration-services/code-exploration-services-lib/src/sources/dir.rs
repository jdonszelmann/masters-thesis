
use std::{fs, io};
use std::cell::RefCell;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::analysis::dir::Analysis;
use crate::analysis::file::FileAnalysis;
use crate::input::AnalysisError;
use crate::languages::{IntoSourceDirError, Language};
use crate::sources::hash::SourceCodeHash;
use crate::sources::span::Span;

#[derive(Debug, Error)]
pub enum NewDirError {
    #[error("walking through directory")]
    Io(#[from] walkdir::Error)
}


#[derive(Debug, Error)]
pub enum NewSingleFileError {
    #[error("file path has no extension")]
    NoExtension(PathBuf),

    #[error("create source dir from single file")]
    CreateDirFromFile(#[from] IntoSourceDirError),
}

#[derive(Debug, Error)]
pub enum ContentsError {
    #[error("read source file contents")]
    Io(#[from] io::Error)
}

#[derive(Debug, Error)]
pub enum NewSourceDirError {
    #[error(transparent)]
    NewSingleFile(#[from] NewSingleFileError),

    #[error(transparent)]
    NewDir(#[from] NewDirError),

    #[error("can't open project, not a file nor a directory")]
    NotAFileOrDir(PathBuf)
}

pub enum FilesList {
    SingleFile(InternalSourceFile),
    Many(Vec<InternalSourceFile>),
}

pub struct SourceDir {
    root: PathBuf,
    files: FilesList,
    cleanup: Option<Box<dyn FnOnce()>>,
}

impl Drop for SourceDir {
    fn drop(&mut self) {
        self.cleanup.take().map(|i| i());
    }
}

impl SourceDir {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, NewSourceDirError> {
        if root.as_ref().is_file() {
            Ok(Self::new_single_file(root)?)
        } else if root.as_ref().is_dir() {
            Ok(Self::new_dir(root)?)
        } else {
            Err(NewSourceDirError::NotAFileOrDir(root.as_ref().to_path_buf()))
        }
    }


    pub fn new_dir(root: impl AsRef<Path>) -> Result<Self, NewDirError> {
        let root = root.as_ref().to_path_buf();
        let files = walkdir::WalkDir::new(&root)
            .into_iter()
            .map(|i| i.map(|i| {
                InternalSourceFile::new(i.path())
            }))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            root,
            files: FilesList::Many(files),
            cleanup: None,
        })
    }

    pub fn new_single_file(file: impl AsRef<Path>) -> Result<Self, NewSingleFileError> {
        let ext = file.as_ref().extension()
            .ok_or_else(|| NewSingleFileError::NoExtension(file.as_ref().to_path_buf()))?;

        let language = Language::from_extension(&ext.to_string_lossy());
        language.source_file_into_dir(file).map_err(Into::into)
    }

    pub(crate) fn __internal_construct_single_file(root: impl AsRef<Path>, file: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();

        Self {
            root,
            files: FilesList::SingleFile(InternalSourceFile::new(file)),
            cleanup: None,
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn set_cleanup(&mut self, cleanup: impl FnOnce() + 'static) {
        self.cleanup = Some(Box::new(cleanup));
    }

    pub fn files(&self) -> FileIter {
        FileIter {
            dir: self,
            loc: 0,
        }
    }

    pub fn map_analyze<'a>(&'a self, mut f: impl FnMut(SourceFile) -> Result<FileAnalysis, AnalysisError>) -> Result<Analysis, AnalysisError> {
        let mut res = Analysis::new();
        for elem in self.files().map(|i| -> Result<(SourceFile, FileAnalysis), AnalysisError> {
            Ok((i, f(i)?))
        }) {
            let (f, a) = elem?;

            res.add_file(f, a);
        }

        Ok(res)
    }
}

pub struct FileIter<'a> {
    dir: &'a SourceDir,
    loc: usize,
}

impl<'a> Iterator for FileIter<'a> {
    type Item = SourceFile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir.files {
            FilesList::Many(ref files) => {
                let elem = files.get(self.loc)?;
                self.loc += 1;

                Some(SourceFile {
                    internal: elem,
                    dir: self.dir,
                })
            }
            FilesList::SingleFile(ref f) => {
                if self.loc == 0 {
                    self.loc = 1;
                    Some(SourceFile {
                        internal: f,
                        dir: self.dir,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Default)]
pub struct InternalSourceFileCache {
    hash: Option<SourceCodeHash>,
    contents: Option<String>,
}

pub struct InternalSourceFile {
    path: PathBuf,
    cache: RefCell<InternalSourceFileCache>
}

impl InternalSourceFile {
    fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            cache: RefCell::new(InternalSourceFileCache::default()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn contents(&self) -> Result<String, ContentsError> {
        if let Some(ref i) = self.cache.borrow().contents {
            return Ok(i.clone());
        }

        let contents = fs::read_to_string(&self.path)?;
        self.cache.borrow_mut().contents = Some(contents.clone());

        Ok(contents)
    }

    pub fn hash(&self) -> Result<SourceCodeHash, HashError> {
        Ok(SourceCodeHash::of(&self.contents()?))
    }

    pub fn slice(&self, span: &Span) -> Result<String, ContentsError> {
        Ok(self.contents()?
            .chars()
            .into_iter()
            .skip(span.start)
            .take(span.len)
            .collect())
    }

    pub fn offset_of_line_num(&self, mut line_num: usize) -> Result<Option<usize>, ContentsError> {
        for (idx, i) in self.contents()?.bytes().enumerate() {
            if i == b'\n' {
                line_num -= 1;
                if line_num <= 1 {
                    return Ok(Some(idx));
                }
            }
        }

        Ok(None)
    }
}

#[derive(Debug, Error)]
pub enum HashError {
    #[error("read file contents")]
    Contents(#[from] ContentsError)
}

#[derive(Copy, Clone)]
pub struct SourceFile<'a> {
    internal: &'a InternalSourceFile,
    dir: &'a SourceDir,
}

impl<'a> SourceFile<'a> {
    fn root(&self) -> &'a SourceDir {
        self.dir
    }
}

impl<'a> Deref for SourceFile<'a> {
    type Target = InternalSourceFile;

    fn deref(&self) -> &Self::Target {
        self.internal
    }
}