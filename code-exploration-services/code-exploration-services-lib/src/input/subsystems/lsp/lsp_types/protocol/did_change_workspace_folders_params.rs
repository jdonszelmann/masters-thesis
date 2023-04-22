use super::WorkspaceFoldersChangeEvent;

/// The parameters of a `workspace/didChangeWorkspaceFolders` notification.
pub struct DidChangeWorkspaceFoldersParams {
	/// The actual workspace folder change event.
	pub event: WorkspaceFoldersChangeEvent,
}
