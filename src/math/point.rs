use crate::math::number::FloatNumber;
use num_traits::Zero;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A point in n-dimensional space.
pub trait Point<F: FloatNumber>:
    Clone
    + Copy
    + Debug
    + Zero
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<F>
    + MulAssign<F>
    + Div<F>
    + DivAssign<F>
{
    /// Returns the dimension of this point.
    fn dim(&self) -> usize;

    /// Returns the vec representation of this point.
    fn to_vec(&self) -> Vec<F>;
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Point2<F: FloatNumber>(pub F, pub F);

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Point3<F: FloatNumber>(pub F, pub F, pub F);

macro_rules! impl_point {
  ($Point:ident { $($label:tt: $field:tt),+ }, $size:expr) => {
    impl<F> $Point<F> where F: FloatNumber {
        /// Create a new vector
        #[inline]
        #[allow(unused)]
        pub fn new($($label: F),+) -> Self {
            Self { $($field: $label),+ }
        }
    }

    impl<F> Display for $Point<F> where F: FloatNumber + Display {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}{:?}", stringify!($Point), ($(self.$field),+))
        }
    }

    impl<F> Point<F> for $Point<F> where F: FloatNumber {
        #[inline]
        fn dim(&self) -> usize {
           $size
        }

        #[inline]
        fn to_vec(&self) -> Vec<F> {
            vec![$(self.$field),+]
        }
    }

    impl<F> Zero for $Point<F> where F: FloatNumber {
        #[inline]
        fn zero() -> Self {
            Self { $($field: F::zero()),+ }
        }

        fn is_zero(&self) -> bool {
            $(self.$field.is_zero()) &&+
        }
    }

    impl<F> Add for $Point<F> where F: FloatNumber {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self { $($field: self.$field + rhs.$field),+ }
        }
    }

    impl<F> Sub for $Point<F> where F: FloatNumber {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Self { $($field: self.$field - rhs.$field),+ }
        }
    }

    impl<F> Mul<F> for $Point<F> where F: FloatNumber {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: F) -> Self::Output {
            Self { $($field: self.$field * rhs),+ }
        }
    }

    impl<F> Div<F> for $Point<F> where F: FloatNumber {
        type Output = Self;

        #[inline]
        fn div(self, divisor: F) -> Self::Output {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            Self { $($field: self.$field / divisor),+ }
        }
    }

    impl<F> AddAssign<$Point<F>> for $Point<F> where F: FloatNumber {
        #[inline]
        fn add_assign(&mut self, rhs: $Point<F>) {
            $(self.$field += rhs.$field);+
        }
    }

    impl<F> SubAssign<$Point<F>> for $Point<F> where F: FloatNumber {
        #[inline]
        fn sub_assign(&mut self, rhs: $Point<F>) {
            $(self.$field -= rhs.$field);+
        }
    }

    impl<F> MulAssign<F> for $Point<F> where F: FloatNumber {
        #[inline]
        fn mul_assign(&mut self, rhs: F) {
            $(self.$field *= rhs);+
        }
    }

    impl<F> DivAssign<F> for $Point<F> where F: FloatNumber {
        #[inline]
        fn div_assign(&mut self, divisor: F) {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            $(self.$field /= divisor);+
        }
    }
  }
}

impl_point!(Point2 { x: 0, y: 1 }, 2);
impl_point!(Point3 { x: 0, y: 1, z: 2 }, 3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dim_should_return_dimension() {
        assert_eq!(Point2::new(1.0, 2.0).dim(), 2);
        assert_eq!(Point3::new(1.0, 2.0, 3.0).dim(), 3);
    }

    #[test]
    fn to_vec_should_return_vec_representation() {
        assert_eq!(Point2::new(1.0, 2.0).to_vec(), vec![1.0, 2.0]);
        assert_eq!(Point3::new(1.0, 2.0, 3.0).to_vec(), vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        assert_eq!(Point2::new(1.0, 2.0).to_string(), "Point2(1.0, 2.0)");
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0).to_string(),
            "Point3(1.0, 2.0, 3.0)"
        );
    }

    #[test]
    fn add_should_add_other_point() {
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 3.0);
        assert_eq!(point1 + point2, Point2::new(3.0, 5.0));

        let point1 = &Point3::new(1.0, 2.0, 3.0);
        let point2 = &Point3::new(2.0, 3.0, 5.0);
        assert_eq!(point1.add(*point2), Point3::new(3.0, 5.0, 8.0));
    }

    #[test]
    fn add_assign_should_add_assign_other() {
        let mut point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 3.0);
        point1.add_assign(point2);
        assert_eq!(point1, Point2::new(3.0, 5.0));
    }

    #[test]
    fn sub_should_sub_other_point() {
        let point1 = Point2::new(1.0, 3.0);
        let point2 = Point2::new(2.0, 2.0);
        assert_eq!(point1 - point2, Point2::new(-1.0, 1.0));

        let point1 = &Point3::new(3.0, 5.0, 7.0);
        let point2 = &Point3::new(1.0, 2.0, 3.0);
        assert_eq!(point1.sub(*point2), Point3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn mul_should_mul_by_scalar() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point * 2.0, Point2::new(2.0, 6.0));

        let point = &Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point.mul(0.5), Point3::new(1.5, 2.5, 3.5));
    }

    #[test]
    fn div_should_div_by_scalar() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point / 2.0, Point2::new(0.5, 1.5));

        let point = &Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point.div(0.5), Point3::new(6.0, 10.0, 14.0));
    }
}
