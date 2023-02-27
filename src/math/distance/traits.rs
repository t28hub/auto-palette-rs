use crate::math::number::FloatNumber;
use crate::math::point::Point;

/// A trait that computes the distance between two points.
pub trait DistanceMeasure {
    /// Compute the distance between two points.
    fn measure<F: FloatNumber, const N: usize>(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F;
}
