use std::ops::Sub;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use crate::math::distance::Distance;

/// A distance for computing euclidean distance.
#[derive(Default)]
pub struct EuclideanDistance {
    squared: SquaredEuclideanDistance,
}

impl<F, const N: usize> Distance<F, Point<F, N>> for EuclideanDistance
where
    F: FloatNumber,
{
    fn measure(&self, lhs: Point<F, N>, rhs: Point<F, N>) -> F {
        let squared = self.squared.measure(lhs, rhs);
        squared.sqrt()
    }
}

impl<F, const N: usize> Distance<F, &Point<F, N>> for EuclideanDistance
where
    F: FloatNumber,
{
    fn measure(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F {
        let squared = self.squared.measure(lhs, rhs);
        squared.sqrt()
    }
}

/// A distance for computing squared euclidean distance.
#[derive(Default)]
pub struct SquaredEuclideanDistance {}

impl<F, const N: usize> Distance<F, Point<F, N>> for SquaredEuclideanDistance
where
    F: FloatNumber,
{
    fn measure(&self, lhs: Point<F, N>, rhs: Point<F, N>) -> F {
        return lhs
            .sub(rhs)
            .to_vec()
            .iter()
            .fold(F::zero(), |total, delta| total + delta.powi(2));
    }
}

impl<F, const N: usize> Distance<F, &Point<F, N>> for SquaredEuclideanDistance
where
    F: FloatNumber,
{
    fn measure(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F {
        return lhs
            .sub(rhs)
            .to_vec()
            .iter()
            .fold(F::zero(), |total, delta| total + delta.powi(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::{Point2, Point3};

    #[test]
    fn compute_should_compute_euclidean_distance() {
        let euclidean = EuclideanDistance::default();
        assert_eq!(
            euclidean.measure(Point2::new(0.0, 1.0), Point2::new(1.0, 0.0)),
            2.0_f32.sqrt()
        );
        assert_eq!(
            euclidean.measure(&Point2::new(0.0, 1.0), &Point2::new(1.0, 0.0)),
            2.0_f32.sqrt()
        );
        assert_eq!(
            euclidean.measure(Point3::new(0.0, 1.0, 2.0), Point3::new(1.0, 2.0, 3.0)),
            3.0_f32.sqrt()
        );
        assert_eq!(
            euclidean.measure(&Point3::new(0.0, 1.0, 2.0), &Point3::new(1.0, 2.0, 3.0)),
            3.0_f32.sqrt()
        );
    }

    #[test]
    fn compute_should_compute_squared_euclidean_distance() {
        let distance = SquaredEuclideanDistance::default();
        assert_eq!(
            distance.measure(Point2::new(0.0, 1.0), Point2::new(1.0, 0.0)),
            2.0
        );
        assert_eq!(
            distance.measure(&Point2::new(0.0, 1.0), &Point2::new(1.0, 0.0)),
            2.0
        );
        assert_eq!(
            distance.measure(Point3::new(0.0, 1.0, 2.0), Point3::new(1.0, 2.0, 3.0)),
            3.0
        );
        assert_eq!(
            distance.measure(&Point3::new(0.0, 1.0, 2.0), &Point3::new(1.0, 2.0, 3.0)),
            3.0
        );
    }
}
