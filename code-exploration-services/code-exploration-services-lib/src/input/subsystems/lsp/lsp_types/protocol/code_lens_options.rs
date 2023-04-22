/// Code Lens options.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CodeLensOptions {
    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,
}
