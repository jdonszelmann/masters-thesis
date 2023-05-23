// mod lsp_types;
mod lsp_communication;
mod lsp_messages;

use crate::analysis::dir::Analysis;
use crate::analysis::field::{Field, FieldRef};
use crate::analysis::file::FileAnalysis;
use crate::input::subsystems::lsp::lsp_communication::{Lsp, NewLspError, RequestError};
use crate::input::{Analyser, AnalysisError};
use crate::languages::Language;
use crate::sources::dir::{SourceDir, SourceFile};
use crate::sources::span::Span;
use lsp_types::notification::{DidOpenTextDocument, Initialized};
use lsp_types::request::{GotoDefinition, Initialize};
use lsp_types::{
    ClientCapabilities, ClientInfo, DidOpenTextDocumentParams, GotoCapability,
    GotoDefinitionParams, GotoDefinitionResponse, InitializeParams, InitializedParams, Position,
    PositionEncodingKind, Range, ReferenceClientCapabilities, TextDocumentClientCapabilities,
    TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams, TraceValue, Url,
    WindowClientCapabilities, WorkspaceClientCapabilities,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::{id, Command, Stdio};
use std::thread;
use std::time::Duration;
use thiserror::Error;
use tracing::info;

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
    fn initialize(
        &mut self,
        parent_id: u32,
        lang_id: &str,
        source: &SourceDir,
    ) -> Result<(), RequestError> {
        // send capabilities
        let resp = self.lsp.request::<Initialize>(&InitializeParams {
            process_id: Some(parent_id),
            root_uri: Some(
                Url::from_file_path(source.root())
                    .map_err(|()| RequestError::ParseUrl(source.root().to_path_buf()))?,
            ),
            capabilities: ClientCapabilities {
                workspace: Some(WorkspaceClientCapabilities {
                    apply_edit: None,
                    workspace_edit: None,
                    did_change_configuration: None,
                    did_change_watched_files: None,
                    symbol: None,
                    execute_command: None,
                    workspace_folders: None,
                    configuration: None,
                    semantic_tokens: None,
                    code_lens: None,
                    file_operations: None,
                    inline_value: None,
                    inlay_hint: None,
                }),
                text_document: Some(TextDocumentClientCapabilities {
                    synchronization: None,
                    completion: None,
                    hover: None,
                    signature_help: None,
                    references: Some(ReferenceClientCapabilities {
                        dynamic_registration: Some(true),
                    }),
                    document_highlight: None,
                    document_symbol: None,
                    formatting: None,
                    range_formatting: None,
                    on_type_formatting: None,
                    declaration: Some(GotoCapability {
                        dynamic_registration: None,
                        link_support: Some(true),
                    }),
                    definition: Some(GotoCapability {
                        dynamic_registration: None,
                        link_support: Some(true),
                    }),
                    type_definition: Some(GotoCapability {
                        dynamic_registration: None,
                        link_support: Some(true),
                    }),
                    implementation: Some(GotoCapability {
                        dynamic_registration: None,
                        link_support: Some(true),
                    }),
                    code_action: None,
                    code_lens: None,
                    document_link: None,
                    color_provider: None,
                    rename: None,
                    publish_diagnostics: None,
                    folding_range: None,
                    selection_range: None,
                    linked_editing_range: None,
                    call_hierarchy: None,
                    semantic_tokens: None,
                    moniker: None,
                    type_hierarchy: None,
                    inline_value: None,
                    inlay_hint: None,
                }),
                window: Some(WindowClientCapabilities {
                    work_done_progress: Some(true),
                    show_message: None,
                    show_document: None,
                }),
                general: None,
                experimental: None,
            },
            initialization_options: None,
            trace: Some(TraceValue::Verbose),
            workspace_folders: None,
            client_info: Some(ClientInfo {
                name: "code exploration services".to_string(),
                version: None,
            }),
            locale: Some("en-US".to_string()),
            ..Default::default()
        })?;
        // verify capabilities
        assert_eq!(
            resp.capabilities.position_encoding,
            Some(PositionEncodingKind::UTF16)
        );

        // send initialized
        self.lsp
            .notification::<Initialized>(&InitializedParams {})?;

        // open documents
        for file in source.files() {
            self.lsp
                .notification::<DidOpenTextDocument>(&DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: Url::from_file_path(file.path())
                            .map_err(|()| RequestError::ParseUrl(file.path().to_path_buf()))?,
                        language_id: lang_id.to_string(),
                        version: 0,
                        text: file.contents()?.to_string(),
                    },
                })?;
        }

        // give the LSP time to respond
        thread::sleep(Duration::from_millis(100));

        // if there was a work progress request, wait for all
        // work to be done
        info!("waiting for LSP ready");
        self.lsp.wait_ready()?;
        info!("LSP ready");

        Ok(())
    }

    fn get_definition_sites(
        &mut self,
        file: SourceFile,
        line: usize,
        character: usize,
    ) -> Result<Vec<Span>, RequestError> {
        let mut res = Vec::new();

        if let Some(resp) = self.lsp.request::<GotoDefinition>(&GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::from_file_path(file.path())
                        .map_err(|()| RequestError::ParseUrl(file.path().to_path_buf()))?,
                },
                position: Position {
                    line: line as u32,
                    character: character as u32,
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })? {
            let span_from_range = |range: Range| -> Result<Option<Span>, RequestError> {
                let start = range.start;
                let end = range.end;

                let Some(start_offset) = file.offset_of_line_num(start.line as usize + 1)? else {
                    return Ok(None);
                };

                let Some(end_offset) = file.offset_of_line_num(end.line as usize + 1)? else {
                    return Ok(None);
                };

                // TODO: handle utf8 well
                Ok(Some(Span::from_start_end(
                    start_offset + start.character as usize,
                    end_offset + end.character as usize,
                )))
            };

            match resp {
                GotoDefinitionResponse::Scalar(s) => {
                    // TODO: handle s.uri
                    if let Some(span) = span_from_range(s.range)? {
                        res.push(span);
                    }
                }
                GotoDefinitionResponse::Array(a) => {
                    for i in a {
                        // TODO: handle s.uri
                        if let Some(span) = span_from_range(i.range)? {
                            res.push(span)
                        }
                    }
                }
                GotoDefinitionResponse::Link(locations) => {
                    for i in locations {
                        // TODO: handle i.uri
                        if let Some(span) = span_from_range(i.target_range)? {
                            res.push(span)
                        }
                    }
                }
            }
        }

        Ok(res)
    }

    fn start_from_path(
        path: &Path,
        lang_id: &str,
        source: &SourceDir,
    ) -> Result<Self, NewLanguageServerError> {
        let mut command = Command::new(path);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());
        command.stderr(Stdio::inherit());

        let mut s = Self {
            lsp: Lsp::new(command.spawn()?)?,
        };
        s.initialize(id(), lang_id, source)?;

        Ok(s)
    }

    pub fn new(ext: &str, source: &SourceDir) -> Result<Self, NewLanguageServerError> {
        let language = Language::from_extension(ext);
        if let Some((lsp_path, lang_id)) = language.lsp() {
            Self::start_from_path(&lsp_path, lang_id, source)
        } else {
            Err(NewLanguageServerError::LanguageNotSupported(
                ext.to_string(),
            ))
        }
    }
}

impl Analyser for LspAnalyser {
    fn symbol_navigation(&self, s: &SourceDir) -> Result<Analysis, AnalysisError> {
        info!("lsp hover documentation");

        let extensions = s
            .files()
            .flat_map(|i| {
                i.path()
                    .extension()
                    .map(|i| i.to_string_lossy().to_string())
            })
            .collect::<HashSet<_>>();

        let mut servers = extensions
            .into_iter()
            .map(|ext| Ok((ext.to_string(), LanguageServer::new(&ext, s)?)))
            .collect::<Result<HashMap<_, _>, _>>()
            .map_err(LanguageServerError::New)?;

        info!("analysing files");

        let analysis = s.map_analyze(|file| {
            let Some(extensions) = file.path().extension() else {
                return Ok(FileAnalysis::new(file, Vec::new())?);
            };
            let Some(server_for_file) = servers.get_mut(&extensions.to_string_lossy().to_string()) else {
                return Ok(FileAnalysis::new(file, Vec::new())?);
            };

            let mut fields = Vec::new();

            let mut line = 0;
            let mut character = 0;
            let mut last = (0, Vec::new());
            for (offset, i) in file.contents()?.chars().enumerate() {
                let resp = server_for_file.get_definition_sites(file, line, character)
                    .map_err(LanguageServerError::Request)?;

                if resp != last.1 {
                    for span in last.1 {
                        fields.push((
                            Span::from_start_end(last.0, offset),
                            Field::Ref {
                                description: "definition".to_string(),
                                reference: FieldRef {
                                    start: span.start,
                                    len: span.len,
                                    next: None,
                                },
                            }
                        ));
                    }

                    last = (offset, resp);
                }

                character += i.len_utf16();
                if i == '\n' {
                    line += 1;
                    character = 0;
                }
            }

            Ok(FileAnalysis::new(file, fields)?)
        })?;

        info!("done");

        Ok(analysis)
    }
}
