use crate::math::distance::DistanceMeasure;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use std::marker::PhantomData;
use std::ops::Sub;

/// A distance for computing euclidean distance.
#[derive(Clone, Debug, PartialEq)]
pub struct EuclideanDistance<F: FloatNumber> {
    squared: SquaredEuclideanDistance<F>,
}

impl<F> Default for EuclideanDistance<F>
where
    F: FloatNumber,
{
    fn default() -> Self {
        Self {
            squared: SquaredEuclideanDistance::default(),
        }
    }
}

impl<F> DistanceMeasure<F> for EuclideanDistance<F>
where
    F: FloatNumber,
{
    fn measure<const N: usize>(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F {
        let squared = self.squared.measure(lhs, rhs);
        squared.sqrt()
    }
}

/// A distance for computing squared euclidean distance.
#[derive(Clone, Debug, PartialEq)]
pub struct SquaredEuclideanDistance<F> {
    _f: PhantomData<F>,
}

impl<F> Default for SquaredEuclideanDistance<F>
where
    F: FloatNumber,
{
    fn default() -> Self {
        Self {
            _f: PhantomData::default(),
        }
    }
}

impl<F> DistanceMeasure<F> for SquaredEuclideanDistance<F>
where
    F: FloatNumber,
{
    fn measure<const N: usize>(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F {
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
            euclidean.measure(&Point2::new(0.0, 1.0), &Point2::new(1.0, 0.0)),
            2.0_f32.sqrt()
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
            distance.measure(&Point2::new(0.0, 1.0), &Point2::new(1.0, 0.0)),
            2.0
        );
        assert_eq!(
            distance.measure(&Point3::new(0.0, 1.0, 2.0), &Point3::new(1.0, 2.0, 3.0)),
            3.0
        );
    }
}
