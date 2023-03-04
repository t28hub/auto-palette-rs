use crate::math::number::FloatNumber;
use std::fmt::{Display, Formatter, Result};

/// Color in standard CIE XYZ color space.
#[derive(Debug, Clone, PartialEq, Copy)]
struct XYZ<F: FloatNumber> {
    pub x: F,
    pub y: F,
    pub z: F,
    pub alpha: F,
}

impl<F> XYZ<F>
where
    F: FloatNumber,
{
    pub fn new(x: F, y: F, z: F, alpha: F) -> XYZ<F> {
        Self { x, y, z, alpha }
    }
}

impl<F> Display for XYZ<F>
where
    F: FloatNumber + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "XYZ({x}, {y}, {z}, {alpha})",
            x = self.x,
            y = self.y,
            z = self.z,
            alpha = self.alpha
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_xyz_color() {
        let xyz = XYZ::new(0.256394, 0.223987, 0.975798, 1.0);
        assert_eq!(xyz.x, 0.256394);
        assert_eq!(xyz.y, 0.223987);
        assert_eq!(xyz.z, 0.975798);
        assert_eq!(xyz.alpha, 1.0);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let xyz = XYZ::new(0.256394, 0.223987, 0.975798, 1.0);
        assert_eq!(xyz.to_string(), "XYZ(0.256394, 0.223987, 0.975798, 1)");
    }
}
