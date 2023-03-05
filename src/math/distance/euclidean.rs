use crate::math::distance::traits::DistanceMeasure;
use crate::math::number::Float;
use crate::math::point::Point;

/// A distance for computing euclidean distance.
#[derive(Clone, Debug, PartialEq)]
pub struct EuclideanDistance;

impl DistanceMeasure for EuclideanDistance {
    fn measure<F: Float, P: Point<F>>(&self, lhs: &P, rhs: &P) -> F {
        SquaredEuclideanDistance.measure(lhs, rhs).sqrt()
    }
}

/// A distance for computing squared euclidean distance.
#[derive(Clone, Debug, PartialEq)]
pub struct SquaredEuclideanDistance;

impl DistanceMeasure for SquaredEuclideanDistance {
    fn measure<F: Float, P: Point<F>>(&self, lhs: &P, rhs: &P) -> F {
        return lhs
            .sub(*rhs)
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
        let euclidean = EuclideanDistance;
        assert_eq!(
            euclidean.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)),
            2.0_f32.sqrt()
        );
        assert_eq!(
            euclidean.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0_f32.sqrt()
        );
    }

    #[test]
    fn compute_should_compute_squared_euclidean_distance() {
        let distance = SquaredEuclideanDistance;
        assert_eq!(distance.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)), 2.0);
        assert_eq!(
            distance.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0
        );
    }
}
