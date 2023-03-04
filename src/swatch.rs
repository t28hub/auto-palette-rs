use std::cmp::Ordering;

/// Color swatch.
#[derive(Clone, Debug, PartialEq)]
pub struct Swatch {
    /// The representative color.
    pub color: (u8, u8, u8),

    /// The position of this swatch.
    pub position: (u32, u32),

    /// The percentage of this swatch.
    pub percentage: f64,
}

impl Eq for Swatch {}

impl PartialOrd for Swatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.percentage.partial_cmp(&other.percentage)
    }
}

impl Ord for Swatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.percentage.total_cmp(&other.percentage)
    }
}
