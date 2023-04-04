pub mod constructor;
pub mod name;
pub mod parser;

mod grammar_definition {
    use serde::{Deserialize, Serialize};
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub type TextmateGrammar = grammar_definition::Root;
impl TextmateGrammar {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn file_types(&self) -> &[String] {
        self.file_types.as_slice()
    }

    pub fn from_language(extension: &str) -> Option<Self> {
        // TODO: get extensions from grammar
        Some(match extension {
            #[cfg(feature = "xml")]
            "json" => {
                Self::from_xml(include_str!("../../textmate_grammars/json.tmLanguage.xml")).unwrap()
            }
            #[cfg(feature = "xml")]
            "css" => {
                Self::from_xml(include_str!("../../textmate_grammars/css.tmLanguage.xml")).unwrap()
            }
            #[cfg(feature = "json")]
            "rs" => Self::from_json(include_str!("../../textmate_grammars/rust.tmLanguage.json"))
                .unwrap(),
            #[cfg(feature = "json")]
            "html" => Self::from_json(include_str!("../../textmate_grammars/html.tmLanguage.json"))
                .unwrap(),
            _ => return None,
        })
    }
}
