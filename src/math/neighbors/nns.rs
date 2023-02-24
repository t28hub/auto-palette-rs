use crate::math::number::FloatNumber;
use std::fmt::{Display, Formatter, Result};

/// Trait to search for neighbors.
pub(crate) trait NearestNeighborSearch<F, T>
where
    F: FloatNumber,
{
    /// Search k-nearest neighbor points.
    fn search(&self, query: T, k: usize) -> Vec<Neighbor<F>>;

    /// Search nearest neighbor point.
    fn search_nearest(&self, query: T) -> Option<Neighbor<F>>;
}

/// A neighbor point.
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub(crate) struct Neighbor<F: FloatNumber> {
    pub index: usize,
    pub distance: F,
}

impl<F> Display for Neighbor<F>
where
    F: FloatNumber + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Neighbor(index={}, distance={})",
            self.index, self.distance
        )
    }
}

impl<F> Neighbor<F>
where
    F: FloatNumber,
{
    /// Create a new neighbor point.
    pub fn new(index: usize, distance: F) -> Self {
        Self { index, distance }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_neighbor() {
        let neighbor = Neighbor::new(3, 2.0.sqrt());
        assert_eq!(
            neighbor,
            Neighbor {
                index: 3,
                distance: 2.0.sqrt()
            }
        );
    }
}
