pub mod grammar;
pub mod theme;

pub use grammar::constructor::{FileType, FromFileError, FromPathError};
pub use grammar::parser::ParseError;
pub use grammar::TextmateGrammar;

#[cfg(test)]
mod tests {
    use crate::TextmateGrammar;

    #[test]
    fn test_rust() {
        let grammar = TextmateGrammar::from_path("textmate_grammars/rust.tmLanguage.json")
            .expect("parse grammar");
        let input = "
fn main() {
    let a = 3;
}
        ";
        let res = grammar.parse(input);

        assert!(res.is_ok(), "{}", res.unwrap_err());

        for (span, name) in res.unwrap() {
            println!(
                "{:?} = {}",
                name,
                input
                    .chars()
                    .skip(span.start)
                    .take(span.len)
                    .collect::<String>()
            )
        }
    }

    #[test]
    fn test_json() {
        let grammar = TextmateGrammar::from_path("textmate_grammars/json.tmLanguage.xml")
            .expect("parse grammar");
        let input = r#"
{"menu": {
  "id": "file",
  "value": "File",
  "popup": {
    "menuitem": [
      {"value": "New", "onclick": "CreateNewDoc()"},
      {"value": "Open", "onclick": "OpenDoc()"},
      {"value": "Close", "onclick": "CloseDoc()"}
    ]
  }
}}
        "#;
        let res = grammar.parse(input);

        assert!(res.is_ok(), "{}", res.unwrap_err());

        for (span, name) in res.unwrap() {
            // println!("{:?}", span);
            println!(
                "{:?} = {}",
                name,
                input
                    .chars()
                    .skip(span.start)
                    .take(span.len)
                    .collect::<String>()
            )
        }
    }
}
