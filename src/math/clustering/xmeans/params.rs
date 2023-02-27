use crate::math::clustering::kmeans::init::Initializer;
use crate::math::distance::DistanceMeasure;
use crate::math::number::FloatNumber;
use rand::Rng;

/// A struct representing the parameters of Xmeans.
#[allow(unused)]
#[derive(Clone, Debug)]
pub(crate) struct XmeansParams<F, D, R>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: Rng + Clone,
{
    max_k: usize,
    max_iterations: usize,
    tolerance: F,
    distance: D,
    initializer: Initializer<R>,
}

impl<F, D, R> XmeansParams<F, D, R>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: Rng + Clone,
{
    #[allow(unused)]
    pub fn new(max_k: usize, distance: D, initializer: Initializer<R>) -> Self {
        Self {
            max_k,
            max_iterations: 10,
            tolerance: F::from_f32(0.0001).expect("Cannot convert tolerance"),
            distance,
            initializer,
        }
    }

    #[allow(unused)]
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    #[allow(unused)]
    pub fn with_tolerance(mut self, tolerance: F) -> Self {
        self.tolerance = tolerance;
        self
    }

    #[allow(unused)]
    pub fn max_k(&self) -> usize {
        self.max_k
    }

    #[allow(unused)]
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    #[allow(unused)]
    pub fn tolerance(&self) -> F {
        self.tolerance
    }

    #[allow(unused)]
    pub fn distance(&self) -> &D {
        &self.distance
    }

    #[allow(unused)]
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
        let params = XmeansParams::new(
            25,
            SquaredEuclideanDistance::default(),
            KmeansPlusPlus(thread_rng()),
        )
        .with_tolerance(0.0125)
        .with_max_iterations(100);
        assert_eq!(params.max_k(), 25);
        assert_eq!(params.tolerance(), 0.0125);
        assert_eq!(params.max_iterations(), 100);
    }
}
