use super::Union;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CancelParams {
    ///The request id to cancel.
    pub id: Union<i32, String>
}
