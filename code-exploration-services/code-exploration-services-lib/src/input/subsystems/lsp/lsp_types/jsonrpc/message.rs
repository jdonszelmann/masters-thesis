/// A language server message
#[derive(Debug, serde::Serialize)]
pub struct Message {
    pub jsonrpc: String,
}
