use crate::math::distance::euclidean::EuclideanDistance;
use crate::math::distance::traits::DistanceMeasure;

/// Parameters of DBSCAN clustering algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct Params<D>
where
    D: DistanceMeasure,
{
    /// The minimum number of neighboring points required for a point to be considered as a core point.
    min_samples: usize,

    /// The minimum number of points required to form a cluster.
    min_cluster_size: usize,

    /// The distance measure to calculate core distances.
    distance: D,
}

impl<D> Params<D>
where
    D: DistanceMeasure,
{
    /// Create a params.
    #[must_use]
    pub fn new(min_samples: usize, min_cluster_size: usize, distance: D) -> Self {
        Self {
            min_samples,
            min_cluster_size,
            distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_params() {
        let params = Params::new(4, 25, EuclideanDistance);
        assert_eq!(params.min_samples, 4);
        assert_eq!(params.min_cluster_size, 25);
        assert_eq!(params.distance, EuclideanDistance);
    }
}
