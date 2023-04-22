use super::Registration;

#[derive(Debug, serde::Serialize)]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}
