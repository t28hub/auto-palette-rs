use crate::math::clustering::hdbscan::core_distance::CoreDistance;
use crate::math::clustering::hdbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::number::Float;
use crate::math::point::Point;

/// HDBSCAN clustering algorithm.
#[derive(Debug, Clone)]
struct HDBSCAN {}

impl HDBSCAN {
    /// Create an HDBSCAN.
    fn new() -> Self {
        Self {}
    }
}

impl<F, P> Fit<F, P, Params> for HDBSCAN
    where
        F: Float,
        P: Point<F>,
{
    fn fit(dataset: &[P], params: &Params) -> Self {
        if dataset.is_empty() {
            return HDBSCAN::new();
        }

        let _core_distance = CoreDistance::new(dataset, params.min_samples(), params.metric());
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}