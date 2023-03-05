use crate::math::number::Float;
use crate::math::point::Point;

pub(crate) trait Fit<F, P, T>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn fit(dataset: &[P], params: &T) -> Self;
}
