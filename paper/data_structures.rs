
// Types
struct Classification(Vec<String>);
struct Span {
    start: usize,
    length: usize,
    file: Path,
}
struct Text(String);

// Relations
enum Relation {
    Outline {
        kind: Classification,
        parent: Option<Span>,
    },
    Syntax {
        kind: Classification,
    },
    Reference {
        target: Span,
        kind: Classification,
    },
    Diagnostics {
        kind: Classification,
        message: Text,
    }
}

// Metadata
type Metadata = Vec<(Span, Relation)>;