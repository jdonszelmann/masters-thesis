use super::{Nullable, ResponseError, Union, Value};

/// A response message.
#[derive(Debug, serde::Deserialize)]
pub struct ResponseMessage {
    pub jsonrpc: String,

    /// The request id.
    pub id: Nullable<Union<i32, String>>,

    /// The result of a request. This can be omitted in
    /// the case of an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    /// The error object in case a request fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError<Value>>,
}
