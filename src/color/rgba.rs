use std::fmt::{Display, Formatter, Result};

/// Color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Create a new RGBA color.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Rgba({r}, {g}, {b}, {a})",
            r = self.r,
            g = self.g,
            b = self.b,
            a = self.a
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_rgba_color() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 64);
        assert_eq!(rgba.b, 255);
        assert_eq!(rgba.a, 128);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.to_string(), "Rgba(0, 64, 255, 128)");
    }
}
