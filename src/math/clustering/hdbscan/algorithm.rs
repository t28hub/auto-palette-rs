use crate::math::clustering::hdbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::distance::traits::DistanceMeasure;
use crate::math::number::Float;
use crate::math::point::Point;

/// HDBSCAN clustering algorithm.
#[derive(Debug, Clone)]
struct HDBSCAN {}

impl<F, P, D> Fit<F, P, Params<D>> for HDBSCAN
where
    F: Float,
    P: Point<F>,
    D: DistanceMeasure,
{
    fn fit(dataset: &[P], params: &Params<D>) -> Self {
        todo!()
    }
}
