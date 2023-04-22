pub struct ConfigurationItem {
	/// The scope to get the configuration section for.
	pub scope_uri: Option<String>,

	/// The configuration section asked for.
	pub section: Option<String>,
}
