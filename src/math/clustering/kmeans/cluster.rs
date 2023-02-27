use crate::math::number::FloatNumber;
use crate::math::point::Point;
use num_traits::Zero;
use std::collections::HashSet;
use std::ops::{AddAssign, DivAssign};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cluster<F, const N: usize>
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
    pub fn new(initial_centroid: &Point<F, N>) -> Self {
        Self {
            centroid: initial_centroid.clone(),
            children: HashSet::new(),
        }
    }

    pub fn centroid(&self) -> &Point<F, N> {
        &self.centroid
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn size(&self) -> usize {
        self.children.len()
    }

    pub fn update_centroid(&mut self) {
        if self.is_empty() {
            self.centroid.set_zero();
        } else {
            let size = F::from_usize(self.children.len()).expect("Cannot convert to FloatNumber");
            self.centroid.div_assign(size);
        }
    }

    pub fn insert(&mut self, index: usize, data: &Point<F, N>) {
        self.centroid.add_assign(data);
        self.children.insert(index);
    }

    pub fn clear(&mut self) {
        self.centroid.set_zero();
        self.children.clear();
    }
}
