use crate::math::number::Float;
use std::cmp::Ordering;

/// Edge of graph.
pub trait Edge {
    /// Return the index of starting vertex.
    #[must_use]
    fn u(&self) -> usize;

    /// Return the index of ending vertex.
    #[must_use]
    fn v(&self) -> usize;
}

/// Weighted edge of graph.
#[derive(Debug, Clone)]
pub struct WeightedEdge<F: Float> {
    u: usize,
    v: usize,
    weight: F,
}

impl<F> WeightedEdge<F>
where
    F: Float,
{
    /// Create a weighted edge.
    pub fn new(u: usize, v: usize, weight: F) -> Self {
        Self { u, v, weight }
    }

    /// Return the weight of this edge.
    pub fn weight(&self) -> F {
        self.weight
    }
}

impl<F> Eq for WeightedEdge<F> where F: Float {}

impl<F> PartialEq for WeightedEdge<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v && self.weight == other.weight
    }
}

impl<F> Ord for WeightedEdge<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Greater)
    }
}

impl<F> PartialOrd for WeightedEdge<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight
            .partial_cmp(&other.weight)
            .map(|ordering| ordering.reverse())
    }
}

impl<F> Edge for WeightedEdge<F>
where
    F: Float,
{
    #[inline]
    fn u(&self) -> usize {
        self.u
    }

    #[inline]
    fn v(&self) -> usize {
        self.v
    }
}

#[cfg(test)]
mod tests {
    use std::f64::NAN;
    use super::*;

    #[test]
    fn new_should_create_weighted_edge() {
        let edge = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge.u(), 0);
        assert_eq!(edge.v(), 1);
        assert_eq!(edge.weight(), 5.0);
    }

    #[test]
    fn eq_should_return_true_if_each_edge_is_equal() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge1.eq(&edge2), true);

        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 2, 5.0);
        assert_eq!(edge1.eq(&edge2), false);
    }

    #[test]
    fn cmp_should_return_reversed_ordering() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Less);

        let edge1 = WeightedEdge::new(0, 1, 2.5);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Equal);

        let edge1 = WeightedEdge::new(0, 1, 2.0);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Greater);

        let edge1 = WeightedEdge::new(0, 1, f64::NAN);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Greater);
    }
}
