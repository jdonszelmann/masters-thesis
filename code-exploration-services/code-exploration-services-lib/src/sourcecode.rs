use crate::analysis::Span;
use once_cell::unsync::OnceCell;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::io;
use std::path::Path;
use std::str::FromStr;
use temp_file::TempFile;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SourceCodeHash(String);
impl SourceCodeHash {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Hash for SourceCodeHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl Display for SourceCodeHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SourceCode {
    contents: String,
    extension: Option<String>,
    hash: OnceCell<SourceCodeHash>,
}

impl FromStr for SourceCode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            contents: s.to_string(),
            extension: None,
            hash: OnceCell::new(),
        })
    }
}

impl SourceCode {
    pub fn as_str(&self) -> &str {
        self.contents.as_str()
    }

    pub fn slice(&self, span: &Span) -> &str {
        &self.contents.as_str()[span.start..span.start + span.len]
    }

    pub fn temp(&self) -> Result<TempFile, io::Error> {
        if let Some(ref i) = self.extension {
            TempFile::with_suffix(format!(".{i}"))
        } else {
            TempFile::new()
        }
        .and_then(|i| i.with_contents(self.contents.as_bytes()))
    }

    pub fn from_str_extension(s: impl AsRef<str>, extension: Option<impl AsRef<str>>) -> Self {
        Self {
            contents: s.as_ref().to_string(),
            extension: extension.map(|i| i.as_ref().to_string()),
            hash: OnceCell::new(),
        }
    }

    pub fn extension(&self) -> Option<&String> {
        self.extension.as_ref()
    }

    pub fn from_path(p: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self::from_str_extension(
            std::fs::read_to_string(&p)?,
            p.as_ref()
                .extension()
                .map(|i| i.to_string_lossy().to_string()),
        ))
    }

    pub fn hash(&self) -> &SourceCodeHash {
        self.hash
            .get_or_init(|| SourceCodeHash(sha256::digest(self.contents.as_str())))
    }

    pub fn offset_of_line_num(&self, mut line_num: usize) -> Option<usize> {
        for (idx, i) in self.contents.bytes().enumerate() {
            if i == b'\n' {
                line_num -= 1;
                if line_num <= 1 {
                    return Some(idx);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::SourceCode;
    use std::io;

    #[test]
    fn test_temp() -> io::Result<()> {
        let input = "test\n\ntest test";
        let s = SourceCode::from_str(input);
        let t = s.temp()?;
        assert_eq!(std::fs::read_to_string(t.path())?, input);

        Ok(())
    }

    #[test]
    fn test_temp_ext() -> io::Result<()> {
        let input = "test\n\ntest test";
        let s = SourceCode::from_str_extension(input, Some("rs"));
        let t = s.temp()?;
        assert_eq!(t.path().extension().unwrap().to_string_lossy(), "rs");

        Ok(())
    }
}
