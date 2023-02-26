use crate::math::clustering::kmeans::algorithm::{Clustering, Kmeans, KmeansParams};
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
    let mut params =
        KmeansParams::new(2, SquaredEuclideanDistance::default(), Random(thread_rng()))
            .with_max_iterations(10);
    let kmeans = Kmeans::fit(&dataset, &mut params);
    println!("{:?}", kmeans.centroids());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        clustering();
    }
}
