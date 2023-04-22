mod lsp_types;
mod lsp_protocol;

use std::io::{Write};
use std::path::Path;
use std::process::{Command, id, Stdio};
use lsp_types::jsonrpc::{Nullable};
use crate::input::{Analyser, AnalysisError};
use crate::{Analysis, SourceCode};
use lsp_types::protocol::{ClientCapabilities, initialize_request, InitializeParams, InitialTraceSetting};
use thiserror::Error;
use crate::input::subsystems::lsp::lsp_protocol::{Lsp, NewLspError, RequestError};
use crate::input::subsystems::lsp::lsp_types::protocol::{did_open_text_document_notification, DidOpenTextDocumentParams, initialized_notification, InitializedParams};
use crate::input::subsystems::lsp::lsp_types::types::TextDocumentItem;

pub struct LspAnalyser;

#[derive(Debug, Error)]
pub enum NewLanguageServerError {
    #[error("spawn lsp command")]
    Spawn(#[from] std::io::Error),

    #[error("start lsp")]
    StartLsp(#[from] NewLspError),

    #[error("request lsp")]
    RequestLsp(#[from] RequestError),

    #[error("language extension not supported")]
    LanguageNotSupported(String),
}

#[derive(Debug, Error)]
pub enum LanguageServerError {
    #[error("initialise lsp")]
    New(#[from] NewLanguageServerError),
    #[error("lsp request")]
    Request(#[from] RequestError),
}

struct LanguageServer {
    lsp: Lsp
}


impl LanguageServer {
    fn initialize(&mut self, parent_id: u32, lang_id: &str, source: &SourceCode, file: &Path) -> Result<(), RequestError> {
        let resp = self.lsp.request(&InitializeParams {
            process_id: parent_id as i32,
            root_path: None,
            root_uri: Nullable::None,
            capabilities: ClientCapabilities {
                workspace: None,
                text_document: None,
                experimental: None,
            },
            initialization_options: None,
            trace: Some(InitialTraceSetting::Verbose),
            workspace_folders: None,
        }, initialize_request::TYPE)?;
        tracing::debug!("{:?}", resp);

        self.lsp.notification(&InitializedParams {}, initialized_notification::TYPE)?;
        self.lsp.notification(&DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: file.to_string_lossy().to_string(),
                language_id: lang_id.to_string(),
                version: 0,
                text: source.as_str().to_string(),
            },
        }, did_open_text_document_notification::TYPE)?;
        Ok(())
    }

    fn start_from_path(path: &str, lang_id: &str, source: &SourceCode, file: &Path) -> Result<Self, NewLanguageServerError> {
        let mut command = Command::new(path);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());

        let mut s = Self {
            lsp: Lsp::new(command.spawn()?)?,
        };
        s.initialize(id(), lang_id, source, file)?;

        Ok(s)
    }

    pub fn new(ext: &str, file: &Path, source: &SourceCode) -> Result<Self, NewLanguageServerError> {
        match ext {
            "rs" => Self::start_from_path("rust-analyzer", "rust", source, file),
            s => Err(NewLanguageServerError::LanguageNotSupported(s.to_string())),
        }
    }
}

impl Analyser for LspAnalyser {
    fn hover_documentation(&self, s: &SourceCode) -> Result<Analysis, AnalysisError> {
        tracing::info!("lsp hover documentation");

        let file = s.temp().expect("temp file");

        let mut lsp = s.extension()
            .map(|i | LanguageServer::new(i, file.path(), s))
            .ok_or(AnalysisError::NotImplemented)?
            .map_err(LanguageServerError::New)?;

        loop {
            lsp.lsp.poll_notification().map_err(LanguageServerError::Request)?;
        }

        // lsp.lsp.request()

        Ok(Analysis::new(s, Vec::new()))
    }
}
