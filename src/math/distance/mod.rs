use crate::math::number::FloatNumber;
use crate::math::point::Point;

pub(crate) mod euclidean;

/// A trait for computing the distance between two points.
pub trait DistanceMeasure<F>: Default
where
    F: FloatNumber,
{
    /// Compute the distance between two points.
    fn measure<const N: usize>(&self, lhs: &Point<F, N>, rhs: &Point<F, N>) -> F;
}
