use super::ConfigurationItem;

/// The parameters of a configuration request.
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}
