/// The parameters of a [WorkspaceSymbolRequest](#WorkspaceSymbolRequest).
#[derive(Debug, serde::Serialize)]
pub struct WorkspaceSymbolParams {
    /// A non-empty query string
    pub query: String,
}
