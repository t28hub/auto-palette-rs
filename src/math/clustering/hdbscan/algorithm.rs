use crate::math::clustering::hdbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::number::Float;
use crate::math::point::Point;

/// HDBSCAN clustering algorithm.
#[derive(Debug, Clone)]
struct HDBSCAN {}

impl<F, P> Fit<F, P, Params> for HDBSCAN
where
    F: Float,
    P: Point<F>,
{
    fn fit(dataset: &[P], params: &Params) -> Self {
        todo!()
    }
}
