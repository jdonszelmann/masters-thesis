use crate::input::subsystems::lsp::lsp_types::jsonrpc::Union;

/// The server supports workspace folder.
///
/// Since 3.6.0
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceFoldersOptions {
    /// The server has support for workspace folders
    pub supported: Option<bool>,

    /// Whether the server wants to receive workspace folder
    /// change notifications.
    ///
    /// If a strings is provided the string is treated as a ID
    /// under which the notification is registered on the client
    /// side. The ID can be used to unregister for these events
    /// using the `client/unregisterCapability` request.
    pub change_notifications: Option<Union<String, bool>>,
}
