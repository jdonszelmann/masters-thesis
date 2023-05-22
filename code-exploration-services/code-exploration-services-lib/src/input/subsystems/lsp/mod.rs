mod lsp_types;
mod lsp_protocol;

use std::path::Path;
use std::process::{Command, id, Stdio};
use crate::input::{Analyser, AnalysisError};
use thiserror::Error;
use crate::analysis::dir::Analysis;
use crate::input::subsystems::lsp::lsp_protocol::{Lsp, NewLspError, RequestError};
use crate::input::subsystems::lsp::lsp_types::jsonrpc::Nullable;
use crate::input::subsystems::lsp::lsp_types::protocol::{ClientCapabilities, definition_request, did_open_text_document_notification, DidOpenTextDocumentParams, initialize_request, initialized_notification, InitializedParams, InitializeParams, InitialTraceSetting, ReferencesCapabilities, SynchronizationCapabilities, TextDocumentClientCapabilities, TextDocumentPositionParams};
use crate::input::subsystems::lsp::lsp_types::types::{Position, TextDocumentIdentifier, TextDocumentItem};
use crate::sources::dir::SourceDir;

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
    lsp: Lsp,
}

impl LanguageServer {
    fn initialize(&mut self, parent_id: u32, lang_id: &str, source: &SourceDir) -> Result<(), RequestError> {
        let resp = self.lsp.request(&InitializeParams {
            process_id: parent_id as i32,
            root_path: None,
            root_uri: Nullable::Some(source.root().to_string_lossy().to_string()),
            capabilities: ClientCapabilities {
                workspace: None,
                text_document: Some(TextDocumentClientCapabilities {
                    synchronization: Some(SynchronizationCapabilities {
                        dynamic_registration: None,
                        will_save: None,
                        will_save_wait_until: None,
                        did_save: None,
                    }),
                    completion: None,
                    hover: None,
                    signature_help: None,
                    references: Some(ReferencesCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    document_highlight: None,
                    document_symbol: None,
                    formatting: None,
                    range_formatting: None,
                    on_type_formatting: None,
                    definition: None,
                    type_definition: None,
                    implementation: None,
                    code_action: None,
                    code_lens: None,
                    document_link: None,
                    color_provider: None,
                    rename: None,
                    publish_diagnostics: None,
                }),
                experimental: None,
            },
            initialization_options: None,
            trace: Some(InitialTraceSetting::Verbose),
            workspace_folders: None,
        }, initialize_request::TYPE)?;
        tracing::debug!("{:?}", resp);

        self.lsp.notification(&InitializedParams {}, initialized_notification::TYPE)?;
        for file in source.files() {
            self.lsp.notification(&DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: format!("file://{}", file.path().to_string_lossy()),
                    language_id: lang_id.to_string(),
                    version: 0,
                    text: file.contents()?.to_string(),
                },
            }, did_open_text_document_notification::TYPE)?;
        }

        Ok(())
    }

    fn get_definition(&mut self, file: &Path, line: usize, character: usize) -> Result<(), RequestError> {
        let resp = self.lsp.request(&TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: format!("file://{}", file.to_string_lossy()) },
            position: Position {
                line: line as i32,
                character: character as i32,
            },
        }, definition_request::TYPE)?;

        println!("{:?}", resp);

        Ok(())
    }

    fn start_from_path(path: &str, lang_id: &str, source: &SourceDir) -> Result<Self, NewLanguageServerError> {
        let mut command = Command::new(path);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());

        let mut s = Self {
            lsp: Lsp::new(command.spawn()?)?,
        };
        s.initialize(id(), lang_id, source)?;

        Ok(s)
    }

    pub fn new(ext: &str, source: &SourceDir) -> Result<Self, NewLanguageServerError> {
        match ext {
            "rs" => Self::start_from_path("rust-analyzer", "rust", source),
            s => Err(NewLanguageServerError::LanguageNotSupported(s.to_string())),
        }
    }
}

impl Analyser for LspAnalyser {
    fn hover_documentation<'a>(&self, s: &'a SourceDir) -> Result<Analysis, AnalysisError> {
        return Err(AnalysisError::NotImplemented)
        // tracing::info!("lsp hover documentation");
        //
        // let mut lsp = s.extension()
        //     .map(|i| LanguageServer::new(i, s))
        //     .ok_or(AnalysisError::NotImplemented)?
        //     .map_err(LanguageServerError::New)?;
        //
        //
        // let fields = Vec::new();
        //
        // let mut line = 0;
        // let mut character = 0;
        // for i in s.as_str().chars() {
        //     let _resp = lsp.get_definition(&file.path(), line, character)
        //         .map_err(LanguageServerError::Request)?;
        //
        //     character += i.len_utf16();
        //     if i == '\n' {
        //         line += 1;
        //         character = 0;
        //     }
        // }
        //
        //
        // // loop {
        // //     lsp.lsp.poll_notification().map_err(LanguageServerError::Request)?;
        // // }
        //
        // Ok(FileAnalysis::new(s, fields))
    }
}
