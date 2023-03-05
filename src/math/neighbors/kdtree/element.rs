use crate::math::number::Float;
use std::cmp::Ordering;
use std::cmp::Ordering::Greater;

/// An index of the node and the distance from a query point.
pub(crate) struct Element<F: Float> {
    index: usize,
    distance: F,
}

impl<F> Element<F>
where
    F: Float,
{
    /// Create a new element.
    pub(crate) fn new(index: usize, distance: F) -> Self {
        Self { index, distance }
    }

    /// Return the node index.
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    /// Return the distance between the node corresponding to the index and the query point.
    pub(crate) fn distance(&self) -> F {
        self.distance
    }
}

impl<F> Eq for Element<F> where F: Float {}

impl<F> PartialEq for Element<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<F> Ord for Element<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Greater)
    }
}

impl<F> PartialOrd for Element<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Return reversed ordering to increase the priority in the BinaryHeap.
        self.distance
            .partial_cmp(&other.distance)
            .map(|ordering| ordering.reverse())
    }
}
