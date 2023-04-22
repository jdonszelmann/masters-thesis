/// Execute command options.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExecuteCommandOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,
}
