use serde::{Deserialize, Serialize};
use std::process::Command;
use std::io::BufRead;
use crate::input::subsystems::ctags::CtagsAnalysisError;
use crate::input::subsystems::ctags::CtagsAnalysisError::RunCtagsCommand;
use crate::SourceCode;

#[derive(Clone)]
pub struct CtagsAnalysis {
    pub tags: Vec<Tag>
}


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TagType {
    Ptag,
    Tag
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Tag {
    pub _type: TagType,
    pub name: String,
    pub path: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub pattern: String,
    pub kind: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename="scopeKind")]
    pub scope_kind: Option<String>,
}

fn parse_json_tag(json: &str) -> Result<Tag, CtagsAnalysisError> {
    Ok(serde_json::from_str(json)?)
}

pub fn run_ctags(s: &SourceCode) -> Result<CtagsAnalysis, CtagsAnalysisError> {
    let mut cmd = Command::new("ctags");
    cmd.arg("--output-format=json");
    cmd.args(["-o", "-"]);
    let f = s.temp()?;
    cmd.arg(f.path());

    let output = cmd.output().map_err(RunCtagsCommand)?;
    if !output.status.success() {
        return Err(CtagsAnalysisError::Ctags(String::from_utf8_lossy(&output.stderr).to_string()));
    }


    Ok(CtagsAnalysis {
        tags: output.stdout.lines()
        .map(|i| Ok::<_, CtagsAnalysisError>(parse_json_tag(&i?)?))
        .collect::<Result<Vec<_>, _>>()?
    })
}
