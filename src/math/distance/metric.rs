use crate::math::number::Float;
use crate::math::point::Point;

/// Distance metric enumerated type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceMetric {
    /// Euclidean distance measure.
    Euclidean,
    /// Squared euclidean distance measure.
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Compute the distance between two points.
    pub fn measure<F: Float, P: Point<F>>(&self, lhs: &P, rhs: &P) -> F {
        match *self {
            DistanceMetric::Euclidean => DistanceMetric::SquaredEuclidean.measure(lhs, rhs).sqrt(),
            DistanceMetric::SquaredEuclidean => lhs
                .sub(*rhs)
                .to_vec()
                .iter()
                .fold(F::zero(), |total, delta| total + delta.powi(2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::{Point2, Point3};

    #[test]
    fn compute_should_compute_euclidean_distance() {
        let metric = DistanceMetric::Euclidean;
        assert_eq!(
            metric.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)),
            2.0_f32.sqrt()
        );
        assert_eq!(
            metric.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0_f32.sqrt()
        );
    }

    #[test]
    fn compute_should_compute_squared_euclidean_distance() {
        let metric = DistanceMetric::SquaredEuclidean;
        assert_eq!(metric.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)), 2.0);
        assert_eq!(
            metric.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0
        );
    }
}
