use crate::math::number::FloatNumber;

pub mod euclidean;

/// A trait for computing the distance between two points.
pub trait Distance<F, T>: Default
    where
        F: FloatNumber,
{
    /// Compute the distance between two points.
    fn measure(&self, lhs: T, rhs: T) -> F;
}
