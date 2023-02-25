use crate::math::clustering::kmeans::algorithm::{Clustering, Kmeans};
use crate::math::clustering::kmeans::initializer::CentroidInitializer::Random;
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::Point2;
use rand::thread_rng;

pub mod image;
mod math;

pub fn clustering() {
    let dataset = vec![
        Point2::new(1.0, 2.0),
        Point2::new(2.0, 1.0),
        Point2::new(2.0, 3.0),
        Point2::new(5.0, 2.0),
        Point2::new(7.0, 3.0),
    ];
    let distance = SquaredEuclideanDistance::default();
    let mut kmeans = Kmeans::new(2, distance, Random(thread_rng()));
    kmeans.fit(&dataset);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        clustering();
    }
}
