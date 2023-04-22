/// General parameters to unregister a request or notification.
#[derive(Debug, serde::Serialize)]
pub struct Unregistration {
    /// The id used to unregister the request or notification. Usually an id
    /// provided during the register request.
    pub id: String,

    /// The method to unregister for.
    pub method: String,
}
