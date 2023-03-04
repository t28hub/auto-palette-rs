use crate::math::clustering::kmeans::init::Initializer;
use crate::math::distance::traits::DistanceMeasure;
use crate::math::number::FloatNumber;
use rand::Rng;

/// A struct representing the parameters of Kmeans.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct KmeansParams<F, D, R>
where
    F: FloatNumber,
    D: DistanceMeasure,
    R: Rng + Clone,
{
    k: usize,
    max_iterations: usize,
    tolerance: F,
    distance: D,
    initializer: Initializer<R>,
}

impl<F, D, R> KmeansParams<F, D, R>
where
    F: FloatNumber,
    D: DistanceMeasure,
    R: Rng + Clone,
{
    pub fn new(k: usize, distance: D, initializer: Initializer<R>) -> Self {
        Self {
            k,
            max_iterations: 10,
            tolerance: F::from_f32(0.0001),
            distance,
            initializer,
        }
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn with_tolerance(mut self, tolerance: F) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn tolerance(&self) -> F {
        self.tolerance
    }

    pub fn distance(&self) -> &D {
        &self.distance
    }

    pub fn initializer(&self) -> &Initializer<R> {
        &self.initializer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::kmeans::init::Initializer::KmeansPlusPlus;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;
    use rand::thread_rng;

    #[test]
    fn should_create_params() {
        let params = KmeansParams::new(5, SquaredEuclideanDistance, KmeansPlusPlus(thread_rng()))
            .with_tolerance(0.025)
            .with_max_iterations(25);
        assert_eq!(params.k(), 5);
        assert_eq!(params.tolerance(), 0.025);
        assert_eq!(params.max_iterations(), 25);
    }
}
