use crate::math::number::FloatNumber;
use crate::math::point::Point;
use std::collections::HashSet;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cluster<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    _t: PhantomData<F>,
    centroid: P,
    children: HashSet<usize>,
}

impl<F, P> Cluster<F, P>
where
    F: FloatNumber,
    P: Point<F>,
{
    pub fn new(initial_centroid: &P) -> Self {
        Self {
            _t: PhantomData::default(),
            centroid: *initial_centroid,
            children: HashSet::new(),
        }
    }

    pub fn centroid(&self) -> &P {
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

    pub fn insert(&mut self, index: usize, data: &P) {
        self.centroid.add_assign(*data);
        self.children.insert(index);
    }

    pub fn clear(&mut self) {
        self.centroid.set_zero();
        self.children.clear();
    }
}
