use super::{Union, Value};

/// Request message
#[derive(Debug, serde::Serialize)]
pub struct RequestMessage {
    pub jsonrpc: String,

    /// The request id.
    pub id: Union<i32, String>,

    /// The method to be invoked.
    pub method: String,

    /// The method's params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}
