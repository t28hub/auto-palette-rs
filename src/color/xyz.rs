use crate::math::number::FloatNumber;
use std::fmt::{Display, Formatter, Result};
use crate::color::rgba::Rgba;

/// Color in standard CIE XYZ color space.
#[derive(Debug, Clone, PartialEq, Copy)]
struct XYZ<F: FloatNumber> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F> XYZ<F>
where
    F: FloatNumber,
{
    #[inline]
    #[must_use]
    pub fn new(x: F, y: F, z: F) -> XYZ<F> {
        Self { x, y, z }
    }

    fn normalize_x(value: F) -> F {
        value.clamp(F::from_f32(0.0), F::from_f32(0.950456))
    }

    fn normalize_y(value: F) -> F {
        value.clamp(F::from_f32(0.0), F::from_f32(1.0))
    }

    fn normalize_z(value: F) -> F {
        value.clamp(F::from_f32(0.0), F::from_f32(1.088644))
    }
}

impl<F> Display for XYZ<F>
where
    F: FloatNumber + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "XYZ({x}, {y}, {z})",
            x = self.x,
            y = self.y,
            z = self.z,
        )
    }
}

impl<F> From<&Rgba> for XYZ<F> where F: FloatNumber {
    #[inline]
    fn from(rgba: &Rgba) -> Self {
        let f = |value: F| -> F {
            if value <= F::from_f32(0.04045) {
                value / F::from_f32(12.92)
            } else {
                ((value + F::from_f32(0.055)) / F::from_f32(1.055)).powf(F::from_f32(2.4))
            }
        };

        let max_value: F = Rgba::max_value();
        let r = f(rgba.r::<F>() / max_value);
        let g = f(rgba.g::<F>() / max_value);
        let b = f(rgba.b::<F>() / max_value);

        let x = F::from_f32(0.412391) * r + F::from_f32(0.357584) * g + F::from_f32(0.180481) * b;
        let y = F::from_f32(0.212639) * r + F::from_f32(0.715169) * g + F::from_f32(0.072192) * b;
        let z = F::from_f32(0.019331) * r + F::from_f32(0.119195) * g + F::from_f32(0.950532) * b;

        return XYZ {
            x: XYZ::normalize_x(x),
            y: XYZ::normalize_y(y),
            z: XYZ::normalize_z(z),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_xyz_color() {
        let xyz = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(xyz.x, 0.256394);
        assert_eq!(xyz.y, 0.223987);
        assert_eq!(xyz.z, 0.975798);
    }

    #[test]
    fn from_rgba_should_convert_to_lab() {
        let black = Rgba::new(0, 0, 0, 255);
        assert_eq!(XYZ::from(&black), XYZ::new(0.0, 0.0, 0.0));

        let white = Rgba::new(255, 255, 255, 255);
        assert_eq!(XYZ::from(&white), XYZ::new(0.9504560232162476, 1.0, 1.088644027709961));

        let red = Rgba::new(255, 0, 0, 255);
        assert_eq!(XYZ::from(&red), XYZ::new(0.41239105587441904, 0.21263902922049316, 0.01933100303881014));

        let green = Rgba::new(0, 255, 0, 255);
        assert_eq!(XYZ::from(&green), XYZ::new(0.35758404205910366, 0.7151690973972887, 0.11919501347900736));

        let blue = Rgba::new(0, 0, 255, 255);
        assert_eq!(XYZ::from(&blue), XYZ::new(0.1804810231477814, 0.07219200692771258, 0.9505321319135721));

        let transparent = Rgba::new(0, 0, 0, 255);
        assert_eq!(XYZ::from(&transparent), XYZ::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let xyz = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(xyz.to_string(), "XYZ(0.256394, 0.223987, 0.975798)");
    }
}
