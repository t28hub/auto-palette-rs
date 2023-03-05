use crate::color::xyz::XYZ;
use crate::math::number::Float;
use std::fmt::{Display, Formatter};

/// Color in CIE L*a*b* color space.
#[derive(Debug, Clone, PartialEq)]
pub struct Lab<F: Float> {
    pub l: F,
    pub a: F,
    pub b: F,
}

impl<F> Lab<F>
where
    F: Float,
{
    /// Create a color in CIE L*a*b* color space.
    #[inline]
    #[must_use]
    pub fn new(l: F, a: F, b: F) -> Self {
        Self {
            l: Self::normalize_l(l),
            a: Self::normalize_a(a),
            b: Self::normalize_b(b),
        }
    }

    /// Return min value of l.
    #[inline]
    #[must_use]
    pub(crate) fn min_l<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Return max value of l.
    #[inline]
    #[must_use]
    pub(crate) fn max_l<T: Float>() -> T {
        T::from_f64(100.0)
    }

    /// Return max value of a.
    #[inline]
    #[must_use]
    pub(crate) fn min_a<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Return max value of a.
    #[inline]
    #[must_use]
    pub(crate) fn max_a<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Return max value of b.
    #[inline]
    #[must_use]
    pub(crate) fn min_b<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Return max value of b.
    #[inline]
    #[must_use]
    pub(crate) fn max_b<T: Float>() -> T {
        T::from_f64(127.0)
    }

    #[inline]
    #[must_use]
    fn normalize_l(value: F) -> F {
        value.clamp(Self::min_l(), Self::max_l())
    }

    #[inline]
    #[must_use]
    fn normalize_a(value: F) -> F {
        value.clamp(Self::min_a(), Self::max_a())
    }

    #[inline]
    #[must_use]
    fn normalize_b(value: F) -> F {
        value.clamp(Self::min_b(), Self::max_b())
    }
}

impl<F> Display for Lab<F>
where
    F: Float + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab({l}, {a}, {b})", l = self.l, a = self.a, b = self.b)
    }
}

impl<F> From<&XYZ<F>> for Lab<F>
where
    F: Float,
{
    #[inline]
    fn from(xyz: &XYZ<F>) -> Self {
        let epsilon = F::from_f64(6.0 / 29.0).powi(3);
        let kappa = F::from_f64(841.0 / 108.0); // ((29.0 / 6.0) ^ 2) / 3.0
        let delta = F::from_f64(4.0 / 29.0);
        let f = |t: F| -> F {
            if t > (epsilon) {
                t.cbrt()
            } else {
                kappa * t + delta
            }
        };

        // TODO: Define D65 struct
        let fx = f(xyz.x / F::from_f64(0.95046));
        let fy = f(xyz.y / F::from_f64(1.0));
        let fz = f(xyz.z / F::from_f64(1.08906));

        let l = F::from_f64(116.0) * fy - F::from_f64(16.0);
        let a = F::from_f64(500.0) * (fx - fy);
        let b = F::from_f64(200.0) * (fy - fz);
        Lab::new(l, a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::rgba::Rgba;

    #[test]
    fn new_should_create_lab_color() {
        let lab = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(
            lab,
            Lab {
                l: 53.23,
                a: 80.11,
                b: 67.22,
            }
        );

        let lab = Lab::new(-4.0, -192.0, -192.0);
        assert_eq!(
            lab,
            Lab {
                l: 0.0,
                a: -128.0,
                b: -128.0,
            }
        );

        let lab = Lab::new(108.0, 128.0, 128.0);
        assert_eq!(
            lab,
            Lab {
                l: 100.0,
                a: 127.0,
                b: 127.0,
            }
        );
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let lab = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(lab.to_string(), "Lab(53.23, 80.11, 67.22)");
    }

    #[test]
    fn from_xyz_should_create_lab_color() {
        let black = XYZ::from(&Rgba::black());
        assert_eq!(Lab::from(&black), Lab::new(0.0, 0.0, 0.0));

        let white = XYZ::from(&Rgba::white());
        assert_eq!(
            Lab::from(&white),
            Lab::new(100.0, -0.0006973446715075049, 0.025466932475515236)
        );

        let red = XYZ::from(&Rgba::red());
        assert_eq!(
            Lab::from(&red),
            Lab::new(53.23711812964011, 80.08964041954192, 67.20313797755821)
        );

        let blue = XYZ::from(&Rgba::blue());
        assert_eq!(
            Lab::from(&blue),
            Lab::new(32.300804117317334, 79.19528089035899, -107.85545119181204)
        );

        let green = XYZ::from(&Rgba::green());
        assert_eq!(
            Lab::from(&green),
            Lab::new(87.73553935045295, -86.18229976985276, 83.18665851299563)
        );
    }
}
