use crate::math::clustering::kmeans::cluster::Cluster;
use crate::math::clustering::kmeans::params::KmeansParams;
use crate::math::clustering::traits::Fit;
use crate::math::distance::traits::DistanceMeasure;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::NeighborSearch;
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use rand::Rng;
use std::marker::PhantomData;

pub struct Kmeans<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    _t: PhantomData<F>,
    clusters: Vec<Cluster<F, P>>,
}

impl<F, P> Kmeans<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    pub(crate) fn centroids(&self) -> Vec<P> {
        self.clusters
            .iter()
            .map(|cluster| -> P { *cluster.centroid() })
            .collect()
    }

    pub(crate) fn count_at(&self, index: usize) -> usize {
        let cluster = self.clusters.get(index);
        cluster.map_or(0, |c| c.size())
    }

    fn reassign<D: DistanceMeasure>(
        dataset: &[P],
        clusters: &mut [Cluster<F, P>],
        distance: &D,
        tolerance: F,
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        let nns = KDTree::new(&centroids, distance);
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

                let difference = distance.measure(&old_centroid, cluster.centroid());
                if difference < tolerance {
                    converged = true;
                }
            });
        converged
    }
}

impl<F, P, D, R> Fit<F, P, KmeansParams<F, D, R>> for Kmeans<F, P>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
    R: Rng + Clone,
{
    fn fit(dataset: &[P], params: &KmeansParams<F, D, R>) -> Self {
        if params.k() == 0 {
            return Self {
                _t: PhantomData::default(),
                clusters: Vec::with_capacity(0),
            };
        }

        if params.k() >= dataset.len() {
            let clusters = dataset
                .iter()
                .enumerate()
                .map(|(index, data)| {
                    let mut cluster = Cluster::new(data);
                    cluster.insert(index, data);
                    cluster
                })
                .collect();
            return Self {
                _t: PhantomData::default(),
                clusters,
            };
        }

        let mut clusters: Vec<Cluster<F, P>> = params
            .initializer()
            .initialize(dataset, params.k(), params.distance())
            .iter()
            .map(|centroid| Cluster::new(centroid))
            .collect();
        for _ in 0..params.max_iterations() {
            let converged = Self::reassign(
                dataset,
                &mut clusters,
                params.distance(),
                params.tolerance(),
            );
            if converged {
                break;
            }
        }
        Kmeans {
            _t: PhantomData::default(),
            clusters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::kmeans::init::Initializer::KmeansPlusPlus;
    use crate::math::clustering::traits::Fit;
    use crate::math::distance::euclidean::SquaredEuclideanDistance;
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn new_should_create_kmeans() {
        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let distance = SquaredEuclideanDistance;
        let initializer = KmeansPlusPlus(thread_rng());
        let mut params = KmeansParams::new(2, distance, initializer);
        let _kmeans = Kmeans::fit(&dataset, &mut params);
    }
}
