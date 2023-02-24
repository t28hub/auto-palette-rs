use crate::math::distance::euclidean::EuclideanDistance;
use crate::math::neighbors::linear::LinearSearch;
use crate::math::neighbors::nns::NearestNeighborSearch;
use crate::math::point::Point2;

mod math;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn neighbors() {
    let dataset = vec![
        Point2::new(1.0, 2.0),
        Point2::new(2.0, 1.0),
        Point2::new(2.0, 3.0),
        Point2::new(5.0, 2.0),
        Point2::new(7.0, 3.0),
    ];
    let nns = LinearSearch::new(&dataset, EuclideanDistance::default());
    if let Some(nearest) = nns.search_nearest(&Point2::new(2.5, 1.2)) {
        println!("Nearest:{}", nearest);
    }
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
        neighbors();
    }
}
