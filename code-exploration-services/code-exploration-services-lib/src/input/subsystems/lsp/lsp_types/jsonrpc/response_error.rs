#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseError<D> {
    /// A number indicating the error type that occurred.
    pub code: i32,

    /// A string providing a short description of the error.
    pub message: String,

    /// A Primitive or Structured value that contains additional
    /// information about the error. Can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<D>
}
