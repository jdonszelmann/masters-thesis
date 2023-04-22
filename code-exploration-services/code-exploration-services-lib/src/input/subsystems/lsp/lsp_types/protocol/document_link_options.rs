/// Document link options
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DocumentLinkOptions {
    /// Document links have a resolve provider as well.
    pub resolve_provider: Option<bool>,
}
