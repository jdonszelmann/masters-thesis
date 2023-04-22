/// Represents a color in RGBA space.
#[derive(Debug, serde::Serialize)]
pub struct Color {
    /// The red component of this color in the range [0-1].
    pub red: f64,

    /// The green component of this color in the range [0-1].
    pub green: f64,

    /// The blue component of this color in the range [0-1].
    pub blue: f64,

    /// The alpha component of this color in the range [0-1].
    pub alpha: f64,
}

/// The Color namespace provides helper functions to work with
/// [Color](#Color) literals.
impl Color {
    /// Creates a new Color literal.
    pub fn create(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}
