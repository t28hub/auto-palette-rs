use crate::math::clustering::dbscan::label::Label;
use crate::math::clustering::dbscan::params::Params;
use crate::math::clustering::traits::Fit;
use crate::math::distance::traits::DistanceMeasure;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::{Neighbor, NeighborSearch};
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;

/// DBSCAN clustering algorithm.
#[derive(Debug, Clone)]
pub struct DBSCAN<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    _t: PhantomData<F>,
    centroids: HashMap<usize, P>,
    membership: HashMap<usize, Vec<usize>>,
    outliers: Vec<usize>,
}

impl<F, P> DBSCAN<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    /// Return a set of centroid.
    pub fn centroids(&self) -> Vec<P> {
        self.centroids
            .values()
            .map(|centroid| centroid.clone())
            .collect()
    }

    /// Count the number of assigned to the given cluster ID.
    pub fn count_at(&self, cluster_id: usize) -> usize {
        self.membership
            .get(&cluster_id)
            .map_or(0, |children| children.len())
    }

    /// Return a set of indices of outliers.
    pub fn outliers(&self) -> Vec<usize> {
        self.outliers.clone()
    }

    fn expand_cluster<D, N>(
        cluster_id: usize,
        dataset: &[P],
        params: &Params<F, D>,
        ns: &N,
        neighbors: &[Neighbor<F>],
        labels: &mut [Label],
    ) where
        D: DistanceMeasure,
        N: NeighborSearch<F, P>,
    {
        let mut queue = VecDeque::new();
        queue.extend(neighbors.iter().map(|n| n.index));
        while let Some(current_index) = queue.pop_front() {
            if labels[current_index].is_assigned() {
                continue;
            }

            if labels[current_index].is_outlier() {
                labels[current_index] = Label::Assigned(cluster_id);
                continue;
            }

            labels[current_index] = Label::Assigned(cluster_id);

            let point = dataset[current_index];
            let secondary_neighbors = ns.search_radius(&point, params.epsilon());
            if secondary_neighbors.len() < params.min_points() {
                continue;
            }

            for secondary_neighbor in secondary_neighbors.into_iter() {
                let secondary_index = secondary_neighbor.index;
                match labels[secondary_index] {
                    Label::Undefined => {
                        labels[secondary_index] = Label::Marked;
                        queue.push_back(secondary_index);
                    }
                    Label::Outlier => {
                        queue.push_back(secondary_index);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<F, P, D> Fit<F, P, Params<F, D>> for DBSCAN<F, P>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    #[must_use]
    fn fit(dataset: &Vec<P>, params: &Params<F, D>) -> Self {
        if dataset.is_empty() {
            return DBSCAN {
                _t: PhantomData::default(),
                centroids: HashMap::new(),
                membership: HashMap::new(),
                outliers: Vec::new(),
            };
        }

        let nns = KDTree::new(dataset, params.distance());
        let mut labels = vec![Label::Undefined; dataset.len()];
        let mut cluster_id: usize = 0;
        for (index, point) in dataset.iter().enumerate() {
            if !labels[index].is_undefined() {
                continue;
            }

            let neighbors = nns.search_radius(point, params.epsilon());
            if neighbors.len() < params.min_points() {
                labels[index] = Label::Outlier;
                continue;
            }

            neighbors.iter().for_each(|neighbor| {
                labels[neighbor.index] = Label::Marked;
            });
            Self::expand_cluster(cluster_id, dataset, params, &nns, &neighbors, &mut labels);
            cluster_id += 1;
        }

        let mut centroids: HashMap<usize, P> = HashMap::new();
        let mut membership: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut outliers: Vec<usize> = Vec::new();
        for (index, label) in labels.into_iter().enumerate() {
            match label {
                Label::Assigned(cluster_id) => {
                    let centroid = {
                        let entry = centroids.entry(cluster_id);
                        entry.or_insert(P::zero())
                    };
                    centroid.add_assign(dataset[index]);

                    let children = {
                        let entry = membership.entry(cluster_id);
                        entry.or_insert(Vec::new())
                    };
                    children.push(index);
                }
                Label::Outlier => outliers.push(index),
                _ => unreachable!(
                    "All points in the dataset are assigned to any cluster or labeled as outlier"
                ),
            }
        }

        for (cluster_id, centroid) in centroids.iter_mut() {
            let Some(children) = membership.get(cluster_id) else {
                continue;
            };
            centroid.div_assign(F::from_usize(children.len()));
        }

        DBSCAN {
            _t: PhantomData::default(),
            centroids,
            membership,
            outliers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::euclidean::EuclideanDistance;
    use crate::math::point::Point2;

    const DATASET: [Point2<f64>; 16] = [
        Point2(0.0, 0.0), // 0
        Point2(0.0, 1.0), // 0
        Point2(0.0, 7.0), // 1
        Point2(0.0, 8.0), // 1
        Point2(1.0, 0.0), // 0
        Point2(1.0, 1.0), // 0
        Point2(1.0, 2.0), // 0
        Point2(1.0, 7.0), // 2
        Point2(1.0, 8.0), // 2
        Point2(2.0, 1.0), // 0
        Point2(2.0, 2.0), // 0
        Point2(4.0, 3.0), // 2
        Point2(4.0, 4.0), // 2
        Point2(4.0, 5.0), // 2
        Point2(5.0, 3.0), // 2
        Point2(5.0, 4.0), // 2
    ];

    #[test]
    fn fit_should_fit_dataset() {
        let dataset = Vec::from(DATASET);
        let params = Params::new(4, 2.0_f64.sqrt(), EuclideanDistance);
        let dbscan = DBSCAN::fit(&dataset, &params);

        let mut centroids = dbscan.centroids();
        centroids.sort_by(|point1, point2| point1.0.total_cmp(&point2.0));
        assert_eq!(
            centroids,
            Vec::from([Point2(0.5, 7.5), Point2(1.0, 1.0), Point2(4.4, 3.8)])
        );
        assert_eq!(dbscan.outliers(), Vec::new());
    }
}
