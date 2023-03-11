use crate::math::number::Float;

#[derive(Debug, PartialEq)]
pub struct Node<F: Float> {
    pub left: usize,
    pub right: usize,
    pub weight: F,
    pub size: usize,
}
