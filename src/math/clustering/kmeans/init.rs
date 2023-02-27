use crate::math::distance::DistanceMeasure;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use rand::{Rng, RngCore};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) enum Initializer<R: RngCore> {
    #[allow(unused)]
    Random(R),
    #[allow(unused)]
    KmeansPlusPlus(R),
}

impl<R> Initializer<R>
where
    R: RngCore,
{
    pub(crate) fn initialize<const N: usize, F: FloatNumber, D: DistanceMeasure<F>>(
        &mut self,
        dataset: &[Point<F, N>],
        k: usize,
        distance: &D,
    ) -> Vec<Point<F, N>> {
        if k == 0 {
            return vec![];
        }
        if k >= dataset.len() {
            let mut centroids = Vec::with_capacity(dataset.len());
            centroids.extend(dataset.iter().cloned());
            return centroids;
        }
        match self {
            Self::Random(rng) => Self::random(dataset, k, rng),
            Self::KmeansPlusPlus(rng) => Self::kmeans_plus_plus(dataset, k, rng, distance),
        }
    }

    fn random<const N: usize, F: FloatNumber>(
        dataset: &[Point<F, N>],
        k: usize,
        rng: &mut R,
    ) -> Vec<Point<F, N>> {
        let mut selected = vec![false; dataset.len()];
        let mut centroids = Vec::with_capacity(k);
        while centroids.len() < k {
            let index = rng.gen_range(0..dataset.len());
            if selected[index] {
                continue;
            }

            let point = dataset.get(index);
            if let Some(centroid) = point {
                selected.insert(index, true);
                centroids.push(centroid.clone());
            }
        }
        centroids
    }

    fn kmeans_plus_plus<const N: usize, F: FloatNumber, D: DistanceMeasure<F>>(
        dataset: &[Point<F, N>],
        k: usize,
        rng: &mut R,
        distance: &D,
    ) -> Vec<Point<F, N>> {
        let mut selected = vec![false; dataset.len()];
        let mut centroids = Vec::with_capacity(k);

        let index = rng.gen_range(0..dataset.len());
        selected.insert(index, true);
        centroids.push(dataset[index].clone());
        while centroids.len() < k {
            let furthest = dataset
                .iter()
                .enumerate()
                .map(|(index, point)| -> (usize, F) {
                    if selected[index] {
                        return (index, F::zero());
                    }

                    let min_distance = centroids
                        .iter()
                        .map(|centroid| distance.measure(point, centroid))
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));
                    if let Some(min) = min_distance {
                        (index, min)
                    } else {
                        (index, F::zero())
                    }
                })
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Greater));

            if let Some((index, _)) = furthest {
                selected.insert(index, true);
                centroids.push(dataset[index].clone());
            } else {
                break;
            }
        }
        centroids
    }
}

#[cfg(test)]
mod tests {
    use crate::math::clustering::kmeans::init::Initializer::{KmeansPlusPlus, Random};
    use crate::math::distance::euclidean::{EuclideanDistance, SquaredEuclideanDistance};
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn random_initialize() {
        let dataset = vec![
            Point2::new(1.0, 2.0),
            Point2::new(3.0, 1.0),
            Point2::new(4.0, 5.0),
            Point2::new(5.0, 5.0),
            Point2::new(2.0, 4.0),
        ];
        let mut initializer = Random(thread_rng());
        let result = initializer.initialize(&dataset, 2, &EuclideanDistance::default());
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn kmeans_plus_plus_initialize() {
        let dataset = vec![
            Point2::new(1.0, 2.0),
            Point2::new(3.0, 1.0),
            Point2::new(4.0, 5.0),
            Point2::new(5.0, 5.0),
            Point2::new(2.0, 4.0),
        ];
        let mut initializer = KmeansPlusPlus(thread_rng());
        let result = initializer.initialize(&dataset, 2, &SquaredEuclideanDistance::default());
        assert_eq!(result.len(), 2);
    }
}
