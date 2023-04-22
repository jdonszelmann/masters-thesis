pub trait TextDocumentIdentifier {
    fn get_uri(&self) -> String;
    fn set_uri(&mut self, uri: String);
}

impl TextDocumentIdentifier for super::TextDocumentIdentifier {
    fn get_uri(&self) -> String {
        self.uri.to_owned()
    }

    fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }
}

impl TextDocumentIdentifier for super::VersionedTextDocumentIdentifier {
    fn get_uri(&self) -> String {
        self.uri.to_owned()
    }

    fn set_uri(&mut self, uri: String) {
        self.uri = uri;
    }
}
