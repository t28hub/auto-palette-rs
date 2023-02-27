use crate::math::number::FloatNumber;
use crate::math::point::Point;

pub(crate) trait Fit<F, const N: usize, P>
where
    F: FloatNumber,
{
    fn fit(dataset: &[Point<F, N>], params: &P) -> Self;
}
