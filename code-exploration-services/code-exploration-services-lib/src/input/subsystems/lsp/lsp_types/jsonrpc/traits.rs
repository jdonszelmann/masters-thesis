pub trait Message {
    fn get_jsonrpc(&self) -> String;
    fn set_jsonrpc(&mut self, value: String);
}

impl Message for super::Message {
    fn get_jsonrpc(&self) -> String {
        self.jsonrpc.to_owned()
    }
    fn set_jsonrpc(&mut self, jsonrpc: String) {
        self.jsonrpc = jsonrpc;
    }
}
impl Message for super::RequestMessage {
    fn get_jsonrpc(&self) -> String {
        self.jsonrpc.to_owned()
    }
    fn set_jsonrpc(&mut self, jsonrpc: String) {
        self.jsonrpc = jsonrpc;
    }
}
impl Message for super::ResponseMessage {
    fn get_jsonrpc(&self) -> String {
        self.jsonrpc.to_owned()
    }
    fn set_jsonrpc(&mut self, jsonrpc: String) {
        self.jsonrpc = jsonrpc;
    }
}
impl Message for super::NotificationMessage {
    fn get_jsonrpc(&self) -> String {
        self.jsonrpc.to_owned()
    }
    fn set_jsonrpc(&mut self, jsonrpc: String) {
        self.jsonrpc = jsonrpc;
    }
}
