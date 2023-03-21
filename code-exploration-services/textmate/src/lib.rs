use serde::{Serialize, Deserialize};

mod grammar {
    use serde::{Serialize, Deserialize};
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

mod parser;
mod constructor;
mod name;

pub type TextmateGrammar = grammar::Root;

impl TextmateGrammar {
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|i| i.as_str())
    }

    pub fn file_types(&self) -> &[String] {
        self.file_types.as_slice()
    }
}

pub use constructor::{FromPathError, FromFileError, FileType};

#[cfg(test)]
mod tests {
    use crate::TextmateGrammar;

    #[test]
    fn test_rust() {
        let grammar = TextmateGrammar::from_path("../textmate_grammars/rust.tmLanguage.json").unwrap();
        let res = grammar.parse("
fn main() {
    let a = 3;
}
        ");

        assert!(res.is_ok(), "{}", res.unwrap_err());

        println!("{:?}", res.unwrap())
    }

    #[test]
    fn test_json() {
        let grammar = TextmateGrammar::from_xml(r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>fileTypes</key>
	<array>
		<string>json</string>
		<string>sublime-settings</string>
		<string>sublime-menu</string>
		<string>sublime-keymap</string>
		<string>sublime-mousemap</string>
		<string>sublime-theme</string>
		<string>sublime-build</string>
		<string>sublime-project</string>
		<string>sublime-completions</string>
	</array>
	<key>foldingStartMarker</key>
	<string>(?x)       # turn on extended mode
  ^        # a line beginning with
  \s*      # some optional space
  [{\[]    # the start of an object or array
  (?!      # but not followed by
    .*     # whatever
    [}\]]  # and the close of an object or array
    ,?     # an optional comma
    \s*    # some optional space
    $      # at the end of the line
  )
  |        # ...or...
  [{\[]    # the start of an object or array
  \s*      # some optional space
  $        # at the end of the line</string>
	<key>foldingStopMarker</key>
	<string>(?x)     # turn on extended mode
  ^      # a line beginning with
  \s*    # some optional space
  [}\]]  # and the close of an object or array</string>
	<key>keyEquivalent</key>
	<string>^~J</string>
	<key>name</key>
	<string>JSON (Javascript Next)</string>
	<key>patterns</key>
	<array>
		<dict>
			<key>include</key>
			<string>#value</string>
		</dict>
	</array>
	<key>repository</key>
	<dict>
		<key>array</key>
		<dict>
			<key>begin</key>
			<string>\[</string>
			<key>beginCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.array.begin.json</string>
				</dict>
			</dict>
			<key>end</key>
			<string>\]</string>
			<key>endCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.array.end.json</string>
				</dict>
			</dict>
			<key>name</key>
			<string>meta.structure.array.json</string>
			<key>patterns</key>
			<array>
				<dict>
					<key>include</key>
					<string>#value</string>
				</dict>
				<dict>
					<key>match</key>
					<string>,</string>
					<key>name</key>
					<string>punctuation.separator.array.json</string>
				</dict>
				<dict>
					<key>match</key>
					<string>[^\s\]]</string>
					<key>name</key>
					<string>invalid.illegal.expected-array-separator.json</string>
				</dict>
			</array>
		</dict>
		<key>comments</key>
		<dict>
			<key>patterns</key>
			<array>
				<dict>
					<key>begin</key>
					<string>/\*\*</string>
					<key>captures</key>
					<dict>
						<key>0</key>
						<dict>
							<key>name</key>
							<string>punctuation.definition.comment.json</string>
						</dict>
					</dict>
					<key>end</key>
					<string>\*/</string>
					<key>name</key>
					<string>comment.block.documentation.json</string>
				</dict>
				<dict>
					<key>begin</key>
					<string>/\*</string>
					<key>captures</key>
					<dict>
						<key>0</key>
						<dict>
							<key>name</key>
							<string>punctuation.definition.comment.json</string>
						</dict>
					</dict>
					<key>end</key>
					<string>\*/</string>
					<key>name</key>
					<string>comment.block.json</string>
				</dict>
				<dict>
					<key>captures</key>
					<dict>
						<key>1</key>
						<dict>
							<key>name</key>
							<string>punctuation.definition.comment.json</string>
						</dict>
					</dict>
					<key>match</key>
					<string>(//).*$\n?</string>
					<key>name</key>
					<string>comment.line.double-slash.js</string>
				</dict>
			</array>
		</dict>
		<key>constant</key>
		<dict>
			<key>match</key>
			<string>\b(?:true|false|null)\b</string>
			<key>name</key>
			<string>constant.language.json</string>
		</dict>
		<key>number</key>
		<dict>
			<key>match</key>
			<string>(?x)        # turn on extended mode
  -?        # an optional minus
  (?:
    0       # a zero
    |       # ...or...
    [1-9]   # a 1-9 character
    \d*     # followed by zero or more digits
  )
  (?:
    (?:
      \.    # a period
      \d+   # followed by one or more digits
    )?
    (?:
      [eE]  # an e character
      [+-]? # followed by an option +/-
      \d+   # followed by one or more digits
    )?      # make exponent optional
  )?        # make decimal portion optional</string>
			<key>name</key>
			<string>constant.numeric.json</string>
		</dict>
		<key>object</key>
		<dict>
			<key>begin</key>
			<string>\{</string>
			<key>beginCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.dictionary.begin.json</string>
				</dict>
			</dict>
			<key>end</key>
			<string>\}</string>
			<key>endCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.dictionary.end.json</string>
				</dict>
			</dict>
			<key>name</key>
			<string>meta.structure.dictionary.json</string>
			<key>patterns</key>
			<array>
				<dict>
					<key>comment</key>
					<string>the JSON object key</string>
					<key>include</key>
					<string>#objectkey</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#comments</string>
				</dict>
				<dict>
					<key>begin</key>
					<string>:</string>
					<key>beginCaptures</key>
					<dict>
						<key>0</key>
						<dict>
							<key>name</key>
							<string>punctuation.separator.dictionary.key-value.json</string>
						</dict>
					</dict>
					<key>end</key>
					<string>(,)|(?=\})</string>
					<key>endCaptures</key>
					<dict>
						<key>1</key>
						<dict>
							<key>name</key>
							<string>punctuation.separator.dictionary.pair.json</string>
						</dict>
					</dict>
					<key>name</key>
					<string>meta.structure.dictionary.value.json</string>
					<key>patterns</key>
					<array>
						<dict>
							<key>comment</key>
							<string>the JSON object value</string>
							<key>include</key>
							<string>#value</string>
						</dict>
						<dict>
							<key>match</key>
							<string>[^\s,]</string>
							<key>name</key>
							<string>invalid.illegal.expected-dictionary-separator.json</string>
						</dict>
					</array>
				</dict>
				<dict>
					<key>match</key>
					<string>[^\s\}]</string>
					<key>name</key>
					<string>invalid.illegal.expected-dictionary-separator.json</string>
				</dict>
			</array>
		</dict>
		<key>string</key>
		<dict>
			<key>begin</key>
			<string>"</string>
			<key>beginCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.string.begin.json</string>
				</dict>
			</dict>
			<key>end</key>
			<string>"</string>
			<key>endCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.definition.string.end.json</string>
				</dict>
			</dict>
			<key>name</key>
			<string>string.quoted.double.json</string>
			<key>patterns</key>
			<array>
				<dict>
					<key>include</key>
					<string>#stringcontent</string>
				</dict>
			</array>
		</dict>
		<key>objectkey</key>
		<dict>
			<key>begin</key>
			<string>"</string>
			<key>beginCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.support.type.property-name.begin.json</string>
				</dict>
			</dict>
			<key>end</key>
			<string>"</string>
			<key>endCaptures</key>
			<dict>
				<key>0</key>
				<dict>
					<key>name</key>
					<string>punctuation.support.type.property-name.end.json</string>
				</dict>
			</dict>
			<key>name</key>
			<string>support.type.property-name.json</string>
			<key>patterns</key>
			<array>
				<dict>
					<key>include</key>
					<string>#stringcontent</string>
				</dict>
			</array>
		</dict>
		<key>stringcontent</key>
		<dict>
			<key>patterns</key>
			<array>
				<dict>
					<key>match</key>
					<string>(?x)                # turn on extended mode
  \\                # a literal backslash
  (?:               # ...followed by...
    ["\\/bfnrt]     # one of these characters
    |               # ...or...
    u               # a u
    [0-9a-fA-F]{4}) # and four hex digits</string>
					<key>name</key>
					<string>constant.character.escape.json</string>
				</dict>
				<dict>
					<key>match</key>
					<string>\\.</string>
					<key>name</key>
					<string>invalid.illegal.unrecognized-string-escape.json</string>
				</dict>
			</array>
		</dict>
		<key>value</key>
		<dict>
			<key>patterns</key>
			<array>
				<dict>
					<key>include</key>
					<string>#constant</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#number</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#string</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#array</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#object</string>
				</dict>
				<dict>
					<key>include</key>
					<string>#comments</string>
				</dict>
			</array>
		</dict>
	</dict>
	<key>scopeName</key>
	<string>source.json</string>
	<key>uuid</key>
	<string>8f97457b-516e-48ce-83c7-08ae12fb327a</string>
</dict>
</plist>

"#).expect("parse grammar");
        let res = grammar.parse("
fn main() {
    let a = 3;
}
        ");

        assert!(res.is_ok(), "{}", res.unwrap_err());

        println!("{:?}", res.unwrap())
    }
}

