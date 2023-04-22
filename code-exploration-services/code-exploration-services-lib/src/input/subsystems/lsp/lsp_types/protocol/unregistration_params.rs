use super::Unregistration;

#[derive(Debug, serde::Serialize)]
pub struct UnregistrationParams {
	unregisterations: Vec<Unregistration>,
}
