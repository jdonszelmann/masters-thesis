#[derive(Debug, serde::Serialize)]
pub enum WatchKind {
    /// Interested in create events.
    Create = 1,

    /// Interested in change events
    Change = 2,

    /// Interested in delete events
    Delete = 4,
}
