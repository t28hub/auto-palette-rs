use crate::math::number::FloatNumber;
use num_traits::Zero;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, Sub};

/// A point in n-dimensional space.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Point<F: FloatNumber, const N: usize> {
    components: RefCell<Vec<F>>,
}

impl<F, const N: usize> Point<F, N>
where
    F: FloatNumber,
{
    /// Returns the dimension of this point.
    pub fn dim(&self) -> usize {
        N
    }

    /// Returns the vec representation of this point.
    pub fn to_vec(&self) -> Vec<F> {
        self.components.borrow().to_vec()
    }
}

impl<F, const N: usize> Display for Point<F, N>
where
    F: FloatNumber,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Point{}{:?}", N, self.components.borrow())
    }
}

impl<F, const N: usize> Zero for Point<F, N>
where
    F: FloatNumber,
{
    fn zero() -> Self {
        Self {
            components: RefCell::new(vec![F::zero(); N]),
        }
    }

    fn is_zero(&self) -> bool {
        self.components.borrow().iter().all(|value| value.is_zero())
    }
}

impl<F, const N: usize> Add for Point<F, N>
where
    F: FloatNumber,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let components1 = self.components.borrow();
        let components2 = other.components.borrow();
        let components = components1
            .iter()
            .zip(components2.iter())
            .map(|(value1, value2)| *value1 + *value2)
            .collect();
        Self {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Add for &Point<F, N>
where
    F: FloatNumber,
{
    type Output = Point<F, N>;

    fn add(self, other: Self) -> Self::Output {
        let components1 = self.components.borrow();
        let components2 = other.components.borrow();
        let components = components1
            .iter()
            .zip(components2.iter())
            .map(|(value1, value2)| *value1 + *value2)
            .collect();
        Self::Output {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> AddAssign<&Point<F, N>> for Point<F, N>
where
    F: FloatNumber,
{
    fn add_assign(&mut self, rhs: &Point<F, N>) {
        self.components
            .borrow_mut()
            .iter_mut()
            .zip(rhs.components.borrow().iter())
            .for_each(|(value1, value2)| value1.add_assign(*value2));
    }
}

impl<F, const N: usize> Sub for Point<F, N>
where
    F: FloatNumber,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let components1 = self.components.borrow();
        let components2 = other.components.borrow();
        let components = components1
            .iter()
            .zip(components2.iter())
            .map(|(value1, value2)| value1.sub(*value2))
            .collect();
        Self {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Sub for &Point<F, N>
where
    F: FloatNumber,
{
    type Output = Point<F, N>;

    fn sub(self, other: Self) -> Self::Output {
        let components1 = self.components.borrow();
        let components2 = other.components.borrow();
        let components = components1
            .iter()
            .zip(components2.iter())
            .map(|(value1, value2)| value1.sub(*value2))
            .collect();
        Self::Output {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Mul<F> for Point<F, N>
where
    F: FloatNumber,
{
    type Output = Self;

    fn mul(self, scalar: F) -> Self::Output {
        let components1 = self.components.borrow();
        let components = components1.iter().map(|value| value.mul(scalar)).collect();
        Self {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Mul<F> for &Point<F, N>
where
    F: FloatNumber,
{
    type Output = Point<F, N>;

    fn mul(self, scalar: F) -> Self::Output {
        let components1 = self.components.borrow();
        let components = components1.iter().map(|value| value.mul(scalar)).collect();
        Self::Output {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Div<F> for Point<F, N>
where
    F: FloatNumber,
{
    type Output = Self;

    fn div(self, scalar: F) -> Self::Output {
        assert!(!scalar.is_zero());
        let components1 = self.components.borrow();
        let components = components1.iter().map(|value| value.div(scalar)).collect();
        Self {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> Div<F> for &Point<F, N>
where
    F: FloatNumber,
{
    type Output = Point<F, N>;

    fn div(self, scalar: F) -> Self::Output {
        assert!(!scalar.is_zero());
        let components1 = self.components.borrow();
        let components = components1.iter().map(|value| value.div(scalar)).collect();
        Self::Output {
            components: RefCell::new(components),
        }
    }
}

impl<F, const N: usize> DivAssign<F> for Point<F, N>
where
    F: FloatNumber,
{
    fn div_assign(&mut self, scalar: F) {
        assert!(!scalar.is_zero());
        let mut components = self.components.borrow_mut();
        components.iter_mut().for_each(|value| {
            *value /= scalar;
        });
    }
}

pub type Point2<F> = Point<F, 2>;

impl<F> Point2<F>
where
    F: FloatNumber,
{
    pub fn new(x: F, y: F) -> Self {
        Self {
            components: RefCell::new(vec![x, y]),
        }
    }
}

pub type Point3<F> = Point<F, 3>;

impl<F> Point3<F>
where
    F: FloatNumber,
{
    pub fn new(x: F, y: F, z: F) -> Self {
        Self {
            components: RefCell::new(vec![x, y, z]),
        }
    }
}

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
        assert_eq!(Point2::new(1.0, 2.0).to_string(), "Point2[1.0, 2.0]");
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0).to_string(),
            "Point3[1.0, 2.0, 3.0]"
        );
    }

    #[test]
    fn add_should_add_other_point() {
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 3.0);
        assert_eq!(point1 + point2, Point2::new(3.0, 5.0));

        let point1 = &Point3::new(1.0, 2.0, 3.0);
        let point2 = &Point3::new(2.0, 3.0, 5.0);
        assert_eq!(point1 + point2, Point3::new(3.0, 5.0, 8.0));
    }

    #[test]
    fn sub_should_sub_other_point() {
        let point1 = Point2::new(1.0, 3.0);
        let point2 = Point2::new(2.0, 2.0);
        assert_eq!(point1 - point2, Point2::new(-1.0, 1.0));

        let point1 = &Point3::new(3.0, 5.0, 7.0);
        let point2 = &Point3::new(1.0, 2.0, 3.0);
        assert_eq!(point1 - point2, Point3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn mul_should_mul_by_scalar() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point * 2.0, Point2::new(2.0, 6.0));

        let point = &Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point * 0.5, Point3::new(1.5, 2.5, 3.5));
    }

    #[test]
    fn div_should_div_by_scalar() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point / 2.0, Point2::new(0.5, 1.5));

        let point = &Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point / 0.5, Point3::new(6.0, 10.0, 14.0));
    }
}
