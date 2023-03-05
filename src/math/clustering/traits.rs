use crate::math::number::Float;
use crate::math::point::Point;

pub(crate) trait Fit<F, P, T>
where
    F: Float,
    P: Point<F>,
{
    fn fit(dataset: &Vec<P>, params: &T) -> Self;
}
