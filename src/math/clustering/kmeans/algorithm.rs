use crate::math::clustering::kmeans::initializer::CentroidInitializer;
use crate::math::distance::DistanceMeasure;
use crate::math::neighbors::linear::LinearSearch;
use crate::math::neighbors::nns::NearestNeighborSearch;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use num_traits::Zero;
use rand::RngCore;
use std::marker::PhantomData;
use std::ops::{AddAssign, DivAssign};

pub trait Clustering<F, const N: usize>
where
    F: FloatNumber,
{
    fn fit(&mut self, dataset: &[Point<F, N>]) -> Vec<usize>;
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cluster<F, const N: usize>
where
    F: FloatNumber,
{
    centroid: Point<F, N>,
    size: usize,
}

impl<F, const N: usize> Cluster<F, N>
where
    F: FloatNumber,
{
    fn new(centroid: Point<F, N>) -> Self {
        Self { centroid, size: 0 }
    }

    fn insert(&mut self, point: &Point<F, N>) {
        (&self.centroid).add_assign(point);
        self.size.add_assign(1);
    }

    fn clear(&mut self) {
        self.centroid.set_zero();
        self.size.set_zero();
    }
}

pub struct Kmeans<F, D, R, const N: usize>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: RngCore,
{
    _f: PhantomData<F>,
    k: usize,
    max_iter: usize,
    tolerance: F,
    distance: D,
    initializer: CentroidInitializer<R>,
}

impl<F, D, R, const N: usize> Kmeans<F, D, R, N>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: RngCore,
{
    pub(crate) fn new(k: usize, distance: D, initializer: CentroidInitializer<R>) -> Self {
        Self {
            _f: PhantomData::default(),
            k,
            max_iter: 10,
            tolerance: F::from(0.01).unwrap(),
            distance,
            initializer,
        }
    }
}

impl<F, D, R, const N: usize> Clustering<F, N> for Kmeans<F, D, R, N>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: RngCore,
{
    fn fit(&mut self, dataset: &[Point<F, N>]) -> Vec<usize> {
        if self.k == 0 {
            return vec![];
        }

        if self.k >= dataset.len() {
            return vec![0; dataset.len()];
        }

        let mut clusters: Vec<Cluster<F, N>> = self
            .initializer
            .initialize(dataset, self.k, &self.distance)
            .into_iter()
            .map(|centroid| Cluster::new(centroid))
            .collect();
        for _ in 0..self.max_iter {
            let old_centroids = clusters
                .iter_mut()
                .map(|cluster| -> Point<F, N> {
                    let old_centroid = cluster.centroid.clone();
                    cluster.clear();
                    old_centroid
                })
                .collect();

            let nns = LinearSearch::new(&old_centroids, &self.distance);
            dataset.iter().for_each(|data| {
                let result = nns.search_nearest(data);
                if let Some(nearest) = result {
                    let cluster = clusters.get_mut(nearest.index).unwrap();
                    cluster.insert(data);
                }
            });

            let mut converged = false;
            clusters
                .iter_mut()
                .zip(old_centroids.iter())
                .for_each(|(cluster, old_centroid)| {
                    if cluster.size == 0 {
                        return;
                    }

                    cluster
                        .centroid
                        .div_assign(F::from_usize(cluster.size).unwrap());

                    let difference = self.distance.measure(old_centroid, &cluster.centroid);
                    if difference < self.tolerance {
                        converged = true;
                    }
                });

            if converged {
                break;
            }
        }

        println!("{:?}", clusters);
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::kmeans::initializer::CentroidInitializer::KmeansPlusPlus;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn new_should_create_kmeans() {
        let distance = SquaredEuclideanDistance::default();
        let initializer = KmeansPlusPlus(thread_rng());
        let mut kmeans = Kmeans::new(2, distance, initializer);

        let dataset = vec![
            Point2::new(1.0, 2.0),
            Point2::new(3.0, 1.0),
            Point2::new(4.0, 5.0),
            Point2::new(5.0, 5.0),
            Point2::new(2.0, 4.0),
        ];
        kmeans.fit(&dataset);
    }
}
