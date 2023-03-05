use crate::math::distance::traits::DistanceMeasure;
use crate::math::number::Float;

/// Parameters of DBSCAN clustering algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct Params<F, D>
where
    F: Float,
    D: DistanceMeasure,
{
    min_points: usize,
    epsilon: F,
    distance: D,
}

impl<F, D> Params<F, D>
where
    F: Float,
    D: DistanceMeasure,
{
    /// Create a new Params with required parameters.
    #[must_use]
    pub fn new(min_points: usize, epsilon: F, distance: D) -> Self {
        Self {
            min_points,
            epsilon,
            distance,
        }
    }

    /// Return the minimum number of points.
    #[must_use]
    pub fn min_points(&self) -> usize {
        self.min_points
    }

    /// Return the epsilon value.
    #[must_use]
    pub fn epsilon(&self) -> F {
        self.epsilon
    }

    /// Return the distance measure.
    #[must_use]
    pub fn distance(&self) -> &D {
        &self.distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;

    #[test]
    fn new_should_create_params() {
        let params = Params::new(16, 5.0, SquaredEuclideanDistance);
        assert_eq!(
            params,
            Params {
                min_points: 16,
                epsilon: 5.0,
                distance: SquaredEuclideanDistance,
            }
        );
        assert_eq!(params.min_points(), 16);
        assert_eq!(params.epsilon(), 5.0);
        assert_eq!(params.distance(), &SquaredEuclideanDistance);
    }
}
