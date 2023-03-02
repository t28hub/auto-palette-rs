use crate::math::number::FloatNumber;
use crate::math::point::Point;

pub(crate) trait Fit<F, P, T>
where
    F: FloatNumber,
    P: Point<F>,
{
    fn fit(dataset: &Vec<P>, params: &T) -> Self;
}
