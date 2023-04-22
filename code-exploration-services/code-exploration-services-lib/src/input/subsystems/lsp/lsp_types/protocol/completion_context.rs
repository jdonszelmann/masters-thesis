use super::CompletionTriggerKind;

/// Contains additional information about the context in which a completion request is triggered.
#[derive(Debug, serde::Serialize)]
pub struct CompletionContext {
    /// How the completion was triggered.
    pub trigger_kind: CompletionTriggerKind,

    /// The trigger character (a single character) that has trigger code complete.
    /// Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
    pub trigger_character: Option<String>,
}
