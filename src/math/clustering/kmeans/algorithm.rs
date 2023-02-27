use crate::math::clustering::kmeans::init::Initializer;
use crate::math::distance::DistanceMeasure;
use crate::math::neighbors::linear::LinearSearch;
use crate::math::neighbors::nns::NearestNeighborSearch;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use num_traits::Zero;
use rand::Rng;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign};

pub(crate) trait Clustering<F, const N: usize, P>
where
    F: FloatNumber,
{
    fn fit(dataset: &[Point<F, N>], params: &mut P) -> Self;
}

#[derive(PartialEq, Clone, Debug)]
struct Cluster<F, const N: usize>
where
    F: FloatNumber,
{
    centroid: Point<F, N>,
    children: HashSet<usize>,
}

impl<F, const N: usize> Cluster<F, N>
where
    F: FloatNumber,
{
    fn new(initial_centroid: &Point<F, N>) -> Self {
        Self {
            centroid: initial_centroid.clone(),
            children: HashSet::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    fn size(&self) -> usize {
        self.children.len()
    }

    fn update_centroid(&mut self) {
        if self.is_empty() {
            self.centroid.set_zero();
        } else {
            let size = F::from_usize(self.children.len()).expect("Cannot convert to FloatNumber");
            self.centroid.div_assign(size);
        }
    }

    fn insert(&mut self, index: usize, data: &Point<F, N>) {
        self.centroid.add_assign(data);
        self.children.insert(index);
    }

    fn clear(&mut self) {
        self.centroid.set_zero();
        self.children.clear();
    }
}

#[derive(Clone, Debug)]
pub(crate) struct KmeansParams<F, D, R>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: Rng,
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
    D: DistanceMeasure<F>,
    R: Rng,
{
    pub(crate) fn new(k: usize, distance: D, initializer: Initializer<R>) -> Self {
        Self {
            k,
            max_iterations: 10,
            tolerance: F::from_f32(0.01).expect("Invalid tolerance value"),
            distance,
            initializer,
        }
    }

    pub(crate) fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub(crate) fn with_tolerance(mut self, tolerance: F) -> Self {
        self.tolerance = tolerance;
        self
    }
}

pub struct Kmeans<F, const N: usize>
where
    F: FloatNumber,
{
    clusters: Vec<Cluster<F, N>>,
}

impl<F, const N: usize> Kmeans<F, N>
where
    F: FloatNumber,
{
    pub(crate) fn centroids(&self) -> Vec<Point<F, N>> {
        self.clusters
            .iter()
            .map(|cluster| -> Point<F, N> { cluster.centroid.clone() })
            .collect()
    }

    pub(crate) fn count_at(&self, index: usize) -> usize {
        let cluster = self.clusters.get(index);
        cluster.map_or(0, |c| c.size())
    }

    fn reassign<D: DistanceMeasure<F>>(
        dataset: &[Point<F, N>],
        clusters: &mut [Cluster<F, N>],
        distance: &D,
        tolerance: F,
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(cluster.centroid.clone());
            cluster.clear();
        }

        let nns = LinearSearch::new(&centroids, distance);
        dataset.iter().enumerate().for_each(|(index, data)| {
            let result = nns.search_nearest(data);
            if let Some(nearest) = result {
                let cluster = clusters
                    .get_mut(nearest.index)
                    .expect("No cluster is found");
                cluster.insert(index, data);
            }
        });

        let mut converged = false;
        clusters
            .iter_mut()
            .zip(centroids)
            .for_each(|(cluster, old_centroid)| {
                if cluster.is_empty() {
                    return;
                }

                cluster.update_centroid();

                let difference = distance.measure(&old_centroid, &cluster.centroid);
                if difference < tolerance {
                    converged = true;
                }
            });
        converged
    }
}

impl<F, D, R, const N: usize> Clustering<F, N, KmeansParams<F, D, R>> for Kmeans<F, N>
where
    F: FloatNumber,
    D: DistanceMeasure<F>,
    R: Rng,
{
    fn fit(dataset: &[Point<F, N>], params: &mut KmeansParams<F, D, R>) -> Self {
        if params.k == 0 {
            return Self {
                clusters: Vec::with_capacity(0),
            };
        }

        if params.k >= dataset.len() {
            let clusters = dataset
                .iter()
                .enumerate()
                .map(|(index, data)| {
                    let mut cluster = Cluster::new(data);
                    cluster.insert(index, data);
                    cluster
                })
                .collect();
            return Self { clusters };
        }

        let mut clusters: Vec<Cluster<F, N>> = params
            .initializer
            .initialize(dataset, params.k, &params.distance)
            .iter()
            .map(|centroid| Cluster::new(centroid))
            .collect();
        for _ in 0..params.max_iterations {
            let converged =
                Self::reassign(dataset, &mut clusters, &params.distance, params.tolerance);
            if converged {
                break;
            }
        }
        Kmeans { clusters }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::kmeans::init::Initializer::KmeansPlusPlus;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn new_should_create_kmeans() {
        let distance = SquaredEuclideanDistance::default();
        let initializer = KmeansPlusPlus(thread_rng());
        let mut params = KmeansParams::new(2, distance, initializer);

        let dataset = vec![
            Point2::new(1.0, 2.0),
            Point2::new(3.0, 1.0),
            Point2::new(4.0, 5.0),
            Point2::new(5.0, 5.0),
            Point2::new(2.0, 4.0),
        ];
        let _kmeans = Kmeans::fit(&dataset, &mut params);
    }
}
