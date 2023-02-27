/// A swatch struct
#[derive(Clone, Debug, PartialEq)]
pub struct Swatch {
    /// The representative color.
    pub color: (u8, u8, u8),

    /// The percentage of this swatch.
    pub percentage: f64,
}
