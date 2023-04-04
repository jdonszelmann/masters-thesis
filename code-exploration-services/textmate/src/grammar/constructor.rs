use std::fs::File;
use std::io;
use std::io::{Cursor, Read};
use std::path::Path;
use plist::Plist;
use thiserror::Error;
use crate::grammar::TextmateGrammar;

#[derive(Error, Debug)]
pub enum FromFileError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[cfg(feature="json")]
    #[error("json deserialization: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature="yaml")]
    #[error("yaml deserialization: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[cfg(feature="xml")]
    #[error("xml deserialization: {0}")]
    Plist(#[from] plist::Error),
}

#[derive(Error, Debug)]
pub enum FromPathError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[cfg(feature="json")]
    #[error("json deserialization: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature="yaml")]
    #[error("yaml deserialization: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[cfg(feature="xml")]
    #[error("xml deserialization: {0}")]
    Xml(#[from] plist::Error),

    #[error("{0}")]
    NotSupported(String)
}

pub enum FileType {
    #[cfg(feature="json")]
    Json,
    #[cfg(feature="yaml")]
    Yaml,
    #[cfg(feature="xml")]
    Xml,
}

impl TextmateGrammar {
    #[cfg(any(feature="json", feature="yaml", feature="xml"))]
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, FromPathError> {
        let data = std::fs::read_to_string(path.as_ref())?;
        Ok(match path.as_ref().extension().map(|i| i.to_str()).flatten() {
            #[cfg(feature="json")]
            Some("json") => Self::from_json(&data)?,
            #[cfg(feature="yaml")]
            Some("yaml" | "yml") => Self::from_yaml(&data)?,
            #[cfg(feature="xml")]
            Some("xml" | "tmLanguage") => Self::from_xml(&data)?,

            #[cfg(not(feature="json"))]
            Some("json") => return Err(FromPathError::NotSupported("json files are not supported without the json feature".into())),
            #[cfg(not(feature="yaml"))]
            Some("yaml" | "yml") => return Err(FromPathError::NotSupported("yaml files are not supported without the yaml feature".into())),
            #[cfg(not(feature="xml"))]
            Some("xml" | "tmLanguage") => return Err(FromPathError::NotSupported("plist xml files are not supported without the plist feature".into())),

            Some(ext) => return Err(FromPathError::NotSupported(format!("only json, yaml and plist xml files are supported (given the correct features). Not {ext}"))),
            None => return Err(FromPathError::NotSupported("path has no file extension so filetype cannot be determined".into()))
        })
    }

    #[cfg(any(feature="json", feature="yaml"))]
    pub fn from_file(f: &mut File, filetype: FileType) -> Result<Self, FromFileError> {
        let mut data = String::new();
        f.read_to_string(&mut data)?;
        Ok(match filetype {
            #[cfg(feature="json")]
            FileType::Json => Self::from_json(&data)?,
            #[cfg(feature="yaml")]
            FileType::Yaml => Self::from_yaml(&data)?,
            #[cfg(feature="xml")]
            FileType::Xml => Self::from_xml(&data)?
        })
    }

    #[cfg(feature="json")]
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }


    #[cfg(feature="xml")]
    fn convert(plist: plist::Plist) -> serde_json::Value {
        match plist {
            Plist::Array(a) => serde_json::Value::Array(a.into_iter().map(Self::convert).collect()),
            Plist::Dict(d) => serde_json::Value::Object(d.into_iter().map(|(k, v)| (k, Self::convert(v))).collect()),
            Plist::Boolean(b) => serde_json::Value::Bool(b),
            Plist::Data(_) => unimplemented!(),
            Plist::DateTime(_) => unimplemented!(),
            Plist::Real(n) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
            Plist::Integer(i) => serde_json::Value::Number(i.into()),
            Plist::String(s) => serde_json::Value::String(s),
        }
    }

    #[cfg(feature="xml")]
    pub fn from_xml(data: &str) -> Result<Self, plist::Error> {
        let mut cursor = Cursor::new(data.as_bytes());
        let plist = Plist::from_reader(&mut cursor)?;
        let json = Self::convert(plist);

        Ok(serde_json::from_value(json).unwrap())
    }

    #[cfg(feature="yaml")]
    pub fn from_yaml(data: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(data)
    }
}

