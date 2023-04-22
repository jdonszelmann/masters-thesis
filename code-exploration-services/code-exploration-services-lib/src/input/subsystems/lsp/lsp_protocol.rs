use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, ExitStatus};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{NotificationMessage, NotificationType, Nullable, RequestType, Union};
use std::io::Read;
use std::sync::{Arc, Mutex};

// TODO: make my own lsp types, this sucks
type Response = super::lsp_types::jsonrpc::ResponseMessage;
type Request = super::lsp_types::jsonrpc::RequestMessage;

pub enum Header {
    ContentType,
    ContentLength(usize),
}

fn parse_header(maybe_header: &str) -> Option<Header> {
    let maybe_header = maybe_header.trim();
    if let Some(rest) = maybe_header.strip_prefix("Content-Length:") {
        Some(Header::ContentLength(rest.trim().parse().expect("invalid length")))
    } else if let Some(_) = maybe_header.strip_prefix("Content-Type:") {
        Some(Header::ContentType)
    } else {
        None
    }
}

fn receiver(responses_tx: Sender<Result<Response, RequestError>>, stdout: ChildStdout) {
    macro_rules! error {
        ($($tt: tt)*) => {
            match $($tt)* {
                Err(e) => {
                    responses_tx.send(Err(RequestError::ReadResponse(e))).unwrap();
                    break;
                }
                Ok(i) => i,
            }
        };
    }

    let mut reader = BufReader::new(stdout);
    let mut line_buf = String::new();
    let mut data_buf = Vec::new();
    let mut expecting_header = true;
    let mut current_length = None;

    loop {
        if expecting_header {
            line_buf.clear();
            if error!(reader.read_line(&mut line_buf)) == 0 {
                responses_tx.send(Err(RequestError::ChildStdoutDied)).unwrap();
                break;
            }
            tracing::debug!("{line_buf}");
            if let Some(header) = parse_header(&line_buf) {
                match header {
                    Header::ContentType => { /* discard */ }
                    Header::ContentLength(l) => {
                        current_length = Some(l);
                    }
                }
            }
            if line_buf.trim_end_matches("\r\n").is_empty() {
                tracing::debug!("now expecting body of length {current_length:?}");
                expecting_header = false;
            }
        } else if let Some(l) = current_length {
            data_buf.resize(l, 0);
            error!(reader.read_exact(&mut data_buf));

            responses_tx.send(serde_json::from_slice(&data_buf).map_err(RequestError::DeserializeResponse)).unwrap();

            expecting_header = true;
        } else {
            expecting_header = true;
        }
    }
}

#[derive(Debug, Error)]
pub enum NewLspError {
    #[error("child process has no stdin pipe")]
    HasStdin,
    #[error("child process has no stdout pipe")]
    HasStdout,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("lsp server has exited for unknown reason")]
    Exited,
    #[error("lsp server has exited with code {0}")]
    ExitedCode(ExitStatus),
    #[error("lsp server has exited")]
    ExitedBecause(#[source] std::io::Error),

    #[error("failed to send request to lsp")]
    WriteRequest(#[from] std::io::Error),

    #[error("failed to read response")]
    ReadResponse(#[source] std::io::Error),

    #[error("child closed stdout")]
    ChildStdoutDied,

    #[error("failed to serialize request")]
    SerializeRequest(#[from] serde_json::Error),

    #[error("failed to deserialize response")]
    DeserializeResponse(#[source] serde_json::Error),

    #[error("failed to get response from lsp in time (timeout)")]
    Timeout,

    #[error("failed to get response from lsp in time (wrong response id)")]
    Id,

    #[error("failed to get response from lsp in time ({0}): {1}")]
    Lsp(i32, String, String),
}

pub struct Lsp {
    process: Arc<Mutex<Child>>,
    stdin: ChildStdin,
    ids: AtomicI32,
    responses_rx: Receiver<Result<Response, RequestError>>,
    responses_tx: Sender<Result<Response, RequestError>>,
    receiver_thread: Option<JoinHandle<()>>,
    exit: Receiver<RequestError>,
}

impl Lsp {
    pub fn new(mut process: Child) -> Result<Self, NewLspError> {
        let (tx, rx) = channel();

        let stdout = process.stdout.take().ok_or(NewLspError::HasStdout)?;
        let stdin = process.stdin.take().ok_or(NewLspError::HasStdin)?;

        let local_sender = tx.clone();
        let receiver_thread = thread::spawn(move || {
            receiver(local_sender, stdout);
        });

        let process = Arc::new(Mutex::new(process));

        let (exit_tx, exit_rx) = channel();
        let local_process = Arc::clone(&process);
        thread::spawn(move || {
            loop {
                match local_process.lock().unwrap().try_wait() {
                    Ok(Some(i)) => {
                        tracing::info!("lsp exited");
                        exit_tx.send(RequestError::ExitedCode(i)).unwrap();
                    }
                    Err(e) => {
                        tracing::info!("lsp exited");
                        exit_tx.send(RequestError::ExitedBecause(e)).unwrap();
                        return;
                    }
                    Ok(None) => {}
                }
                thread::sleep(Duration::from_millis(500));
            }
        });

        Ok(Self {
            process,
            stdin,
            ids: Default::default(),
            responses_rx: rx,
            responses_tx: tx,
            receiver_thread: Some(receiver_thread),
            exit: exit_rx,
        })
    }

    /// A notification is a request without expected response
    pub fn notification<Req: Serialize, Opt>(&mut self, request: &Req, notify_type: NotificationType<Req, Opt>) -> Result<(), RequestError> {
        self.closed()?;

        let msg = serde_json::to_vec(&NotificationMessage {
            jsonrpc: "2.0".to_string(),
            method: notify_type.method.to_string(),
            params: Some(serde_json::to_value(request).map_err(RequestError::SerializeRequest)?),
        })?;

        self.stdin.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())?;
        self.stdin.write_all(&msg)?;

        Ok(())
    }

    fn closed(&self) -> Result<(), RequestError> {
        match self.exit.try_recv() {
            Ok(i) => Err(i),
            Err(TryRecvError::Disconnected) => Err(RequestError::Exited),
            _ => Ok(())
        }
    }

    /// A request to an LSP is guaranteed to get a response. Requests without responses are called notifications
    pub fn request<Req: Serialize, Res: for<'de> Deserialize<'de>, Err>(&mut self, request: &Req, request_type: RequestType<Req, Res, Err, ()>) -> Result<Res, RequestError> {
        self.closed()?;

        let id = self.ids.fetch_add(1, Ordering::SeqCst);

        let msg = serde_json::to_vec(&Request {
            jsonrpc: "2.0".to_string(),
            id: Union::for0(id),
            method: request_type.method.to_string(),
            params: Some(serde_json::to_value(request).map_err(RequestError::SerializeRequest)?),
        })?;

        self.stdin.write_all(format!("Content-Length: {}\r\n\r\n", msg.len()).as_bytes())?;
        self.stdin.write_all(&msg)?;

        let response = 'outer: {
            let mut wrong_id = 0;
            while let Ok(i) = self.responses_rx.recv_timeout(Duration::from_millis(1000)) {
                let i = i?;
                match i.id {
                    Nullable::Some(Union::A(resp_id)) if resp_id == id => {
                        break 'outer i;
                    }
                    Nullable::Some(Union::B(ref resp_id)) if resp_id == &id.to_string() => {
                        break 'outer i;
                    }
                    _ => {}
                }

                wrong_id += 1;
                if wrong_id >= 3 {
                    return Err(RequestError::Id);
                } else {
                    // if we haven't gotten the wrong one 3x in a row, just send again, maybe something else is waiting for it
                    self.responses_tx.send(Ok(i)).unwrap();
                }
            }

            return Err(RequestError::Timeout);
        };

        if let Some(i) = response.error {
            let data = format!("{:?}", i);
            return Err(RequestError::Lsp(i.code, i.message, data));
        }

        Ok(serde_json::from_value(response.result.expect("no response and no error")).map_err(RequestError::DeserializeResponse)?)
    }

    pub fn poll_notification(&mut self) -> Result<(), RequestError> {
        self.closed()?;

        Ok(())
    }
}

impl Drop for Lsp {
    fn drop(&mut self) {
        let mut process = self.process.lock().unwrap();
        if process.try_wait().expect("failed to find if child process has exited").is_none() {
            self.process.lock().unwrap().kill().expect("kill language server");
        }
        self.receiver_thread.take().map(|i| i.join().expect("LSP receiver thread panicked"));
    }
}
