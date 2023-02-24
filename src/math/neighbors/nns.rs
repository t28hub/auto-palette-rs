use crate::math::number::FloatNumber;
use std::fmt::{Display, Formatter, Result};

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
    pub fn new(index: usize, distance: F) -> Self {
        Self { index, distance }
    }
}

pub(crate) trait NearestNeighborSearch<F, T>
where
    F: FloatNumber,
{
    fn search(&self, query: T, k: usize) -> Vec<Neighbor<F>>;

    fn search_nearest(&self, query: T) -> Option<Neighbor<F>>;
}
