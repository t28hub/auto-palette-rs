use crate::math::number::Float;
use crate::math::point::Point;

/// A trait that computes the distance between two points.
pub trait DistanceMeasure {
    /// Compute the distance between two points.
    fn measure<F: Float, P: Point<F>>(&self, lhs: &P, rhs: &P) -> F;
}
