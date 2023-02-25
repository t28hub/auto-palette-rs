use crate::math::clustering::kmeans::{Clustering, Kmeans};
use crate::math::distance::euclidean::SquaredEuclideanDistance;
use crate::math::point::Point2;
use rand::thread_rng;

mod math;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn clustering() {
    let dataset = vec![
        Point2::new(1.0, 2.0),
        Point2::new(2.0, 1.0),
        Point2::new(2.0, 3.0),
        Point2::new(5.0, 2.0),
        Point2::new(7.0, 3.0),
    ];
    let distance = SquaredEuclideanDistance::default();
    let mut kmeans = Kmeans::new(2, distance, thread_rng());
    kmeans.fit(&dataset);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_neighbors() {
        clustering();
    }
}
