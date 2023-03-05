use crate::math::number::Float;

/// Trait for a white point.
///
/// [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint<F>
where
    F: Float,
{
    #[must_use]
    fn x() -> F;

    #[must_use]
    fn y() -> F;

    #[must_use]
    fn z() -> F;
}

/// CIE standard illuminant D65
///
/// [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Debug, Clone, PartialEq)]
pub struct D65;

impl<F> WhitePoint<F> for D65
where
    F: Float,
{
    #[inline]
    fn x() -> F {
        F::from_f64(0.95046)
    }

    #[inline]
    fn y() -> F {
        F::from_f64(1.0)
    }

    #[inline]
    fn z() -> F {
        F::from_f64(1.08906)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d65() {
        let x: f64 = D65::x();
        assert_eq!(x, 0.95046_f64);

        let y: f64 = D65::y();
        assert_eq!(y, 1.00000_f64);

        let z: f64 = D65::z();
        assert_eq!(z, 1.08906_f64);
    }
}
