use crate::math::distance::traits::DistanceMeasure;
use crate::math::neighbors::nns::{NearestNeighborSearch, Neighbor};
use crate::math::number::FloatNumber;
use crate::math::point::Point;
use std::cmp::Ordering;
use std::cmp::Ordering::Greater;
use std::collections::BinaryHeap;
use std::marker::PhantomData;
use std::ops::Div;

/// A node of KDTree
#[derive(Debug)]
struct Node {
    index: usize,
    axis: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Create a new node.
    fn new(index: usize, axis: usize, left: Option<Node>, right: Option<Node>) -> Self {
        Self {
            index,
            axis,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Return whether this node is leaf.
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// An index of the node and the distance from a query point.
struct NodeIndex<F: FloatNumber> {
    index: usize,
    distance: F,
}

impl<F> Eq for NodeIndex<F> where F: FloatNumber {}

impl<F> PartialEq for NodeIndex<F>
where
    F: FloatNumber,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<F> Ord for NodeIndex<F>
where
    F: FloatNumber,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Greater)
    }
}

impl<F> PartialOrd for NodeIndex<F>
where
    F: FloatNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance
            .partial_cmp(&other.distance)
            .map(|ordering| ordering.reverse())
    }
}

#[derive(Debug)]
pub(crate) struct KDTree<'a, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    _t: PhantomData<F>,
    root: Option<Box<Node>>,
    dataset: &'a Vec<P>,
    distance: D,
}

impl<'a, F, P, D> KDTree<'a, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    /// Create a new KDTree.
    pub fn new(dataset: &'a Vec<P>, distance: D) -> Self {
        let mut indices: Vec<usize> = (0..dataset.len()).collect();
        let root = Self::build_node(dataset, &mut indices, 0);
        KDTree {
            _t: PhantomData::default(),
            root: root.map(Box::new),
            dataset,
            distance,
        }
    }

    fn search_recursively(
        &self,
        root: &Option<Box<Node>>,
        query: &P,
        k: usize,
        heap: &mut BinaryHeap<NodeIndex<F>>,
    ) {
        let Some(node) = root else {
            return;
        };

        let index = node.index;
        let point = self.dataset[index];
        let distance = self.distance.measure(&point, query);
        heap.push(NodeIndex { index, distance });
        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        let distance = heap
            .peek()
            .map(|entry| entry.distance)
            .unwrap_or(F::neg_infinity());
        if heap.len() < k || delta.abs() <= distance {
            self.search_recursively(&node.left, query, k, heap);
            self.search_recursively(&node.right, query, k, heap);
        } else if delta < F::zero() {
            self.search_recursively(&node.left, query, k, heap);
        } else {
            self.search_recursively(&node.right, query, k, heap);
        }
    }

    fn build_node(dataset: &'a [P], indices: &mut [usize], depth: usize) -> Option<Node> {
        if dataset.is_empty() || indices.is_empty() {
            return None;
        }

        let axis = depth % dataset[0].dim();
        indices.sort_unstable_by(|index1, index2| {
            let lhs = dataset[*index1].index(axis);
            let rhs = dataset[*index2].index(axis);
            lhs.partial_cmp(rhs).unwrap_or(Greater)
        });

        let median = indices.len().div(2);
        let node = Node::new(
            indices[median],
            axis,
            Self::build_node(dataset, &mut indices[..median], depth + 1),
            Self::build_node(dataset, &mut indices[median + 1..], depth + 1),
        );
        Some(node)
    }
}

impl<F, P, D> NearestNeighborSearch<F, &P> for KDTree<'_, F, P, D>
where
    F: FloatNumber,
    P: Point<F>,
    D: DistanceMeasure,
{
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k < 1 {
            return Vec::with_capacity(0);
        }

        let mut heap: BinaryHeap<NodeIndex<F>> = BinaryHeap::new();
        self.search_recursively(&self.root, query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while neighbors.len() < k {
            let Some(entry) = heap.pop() else {
                break;
            };
            neighbors.push(Neighbor::new(entry.index, entry.distance));
        }
        neighbors
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
    fn test() {
        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
            Point2(0.0, 5.0),
            Point2(2.0, 1.0),
            Point2(5.0, 2.0),
        ];
        let kdtree = KDTree::new(&dataset, SquaredEuclideanDistance);
        assert_eq!(kdtree.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 1),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 2),
            vec![Neighbor::new(4, 2.0), Neighbor::new(1, 4.0),]
        );
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 10),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(7, 5.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(3, 8.0),
                Neighbor::new(5, 13.0),
            ]
        );
    }
}
