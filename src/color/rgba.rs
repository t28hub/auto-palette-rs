use std::fmt::{Display, Formatter, Result};
use crate::math::number::Number;

/// Color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    const MAX: u8 = u8::MAX;

    /// Return the max value of RGBA.
    #[inline]
    #[must_use]
    pub fn max_value<T: Number>() -> T {
        T::from_u8(Self::MAX)
    }

    /// Create a new RGBA color.
    #[inline]
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Return the value of red.
    #[inline]
    #[must_use]
    pub fn r<T: Number>(&self) -> T {
        T::from_u8(self.r)
    }

    /// Return the value of green.
    #[inline]
    #[must_use]
    pub fn g<T: Number>(&self) -> T {
        T::from_u8(self.g)
    }

    /// Return the value of blue.
    #[inline]
    #[must_use]
    pub fn b<T: Number>(&self) -> T {
        T::from_u8(self.b)
    }

    /// Return the value of alpha.
    #[inline]
    #[must_use]
    pub fn a<T: Number>(&self) -> T {
        T::from_u8(self.a)
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

    #[test]
    fn should_return_value_to_be_cast() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.r::<f64>(), 0.0);
        assert_eq!(rgba.g::<f64>(), 64.0);
        assert_eq!(rgba.b::<f64>(), 255.0);
        assert_eq!(rgba.a::<f64>(), 128.0);
    }
}
