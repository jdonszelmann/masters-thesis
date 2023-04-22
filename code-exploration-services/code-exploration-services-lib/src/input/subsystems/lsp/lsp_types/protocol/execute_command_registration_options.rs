/// Execute command registration options.
#[derive(Debug, serde::Serialize)]
pub struct ExecuteCommandRegistrationOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,
}
