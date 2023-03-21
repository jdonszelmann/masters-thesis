use std::fmt::{Display, Formatter};
use std::io;
use std::path::Path;
use once_cell::unsync::OnceCell;

#[derive(Debug, Eq, PartialEq)]
pub struct SourceCodeHash(String);
impl SourceCodeHash {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for SourceCodeHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SourceCode {
    contents: String,
    hash: OnceCell<SourceCodeHash>
}

impl SourceCode {
    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn from_str(s: impl AsRef<str>) -> Self {
        Self {
            contents: s.as_ref().to_string(),
            hash: OnceCell::new(),
        }
    }

    pub fn from_path(p: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self::from_str(std::fs::read_to_string(p)?))
    }

    pub fn hash(&self) -> &SourceCodeHash {
        self.hash.get_or_init(|| {
            SourceCodeHash(sha256::digest(self.contents.as_str()))
        })
    }
}

