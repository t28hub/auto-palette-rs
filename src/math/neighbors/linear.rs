use crate::math::distance::traits::DistanceMeasure;
use crate::math::neighbors::nns::{NearestNeighborSearch, Neighbor};
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// A nearest neighbor search using linear search.
#[allow(unused)]
pub(crate) struct LinearSearch<'a, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    _t: PhantomData<F>,
    dataset: &'a Vec<P>,
    distance: &'a D,
}

impl<'a, F, P, D> LinearSearch<'a, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    #[allow(unused)]
    pub fn new(dataset: &'a Vec<P>, distance: &'a D) -> Self {
        Self {
            _t: PhantomData::default(),
            dataset,
            distance,
        }
    }
}

impl<'a, F, P, D> NearestNeighborSearch<F, &P> for LinearSearch<'a, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return vec![];
        }

        let mut neighbors: Vec<Neighbor<F>> = self
            .dataset
            .iter()
            .enumerate()
            .map(|(index, point)| -> Neighbor<F> {
                let distance = self.distance.measure(point, query);
                Neighbor::new(index, distance)
            })
            .collect();

        neighbors.sort_unstable_by(|neighbor1, neighbor2| -> Ordering {
            neighbor1.distance.partial_cmp(&neighbor2.distance).unwrap()
        });

        let mut results = Vec::with_capacity(k);
        results.extend(neighbors.into_iter().take(k));
        results
    }

    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;
    use crate::math::point::Point2;

    #[test]
    fn search_should_return_neighbors() {
        let dataset = vec![];
        let distance = SquaredEuclideanDistance;
        let linear_search = LinearSearch::new(&dataset, &distance);
        assert_eq!(linear_search.search(&Point2(3.0, 3.0), 0), vec![]);

        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let linear_search = LinearSearch::new(&dataset, &distance);
        assert_eq!(linear_search.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 3),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
            ]
        );
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 5),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(3, 8.0),
            ]
        );
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 6),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(3, 8.0),
            ]
        );
    }

    #[test]
    fn search_nearest_should_return_nearest_neighbor() {
        let dataset = vec![];
        let distance = SquaredEuclideanDistance;
        let linear_search = LinearSearch::new(&dataset, &distance);
        assert_eq!(linear_search.search_nearest(&Point2(0.0, 1.0)), None);

        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let linear_search = LinearSearch::new(&dataset, &distance);
        assert_eq!(
            linear_search.search_nearest(&Point2(2.0, 3.0)),
            Some(Neighbor::new(3, 1.0))
        );
    }
}
