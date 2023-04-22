/// Static registration options to be returned in the initialize
/// request.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StaticRegistrationOptions {
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}
