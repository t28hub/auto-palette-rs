/// A node of KDTree
#[derive(Debug)]
pub(crate) struct Node {
    index: usize,
    axis: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Create a new node.
    pub(crate) fn new(index: usize, axis: usize, left: Option<Node>, right: Option<Node>) -> Self {
        Self {
            index,
            axis,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Return the index of this node.
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    /// Return the axis when this node is split.
    pub(crate) fn axis(&self) -> usize {
        self.axis
    }

    /// Return the left node of this node.
    pub(crate) fn left(&self) -> Option<&Box<Node>> {
        self.left.as_ref()
    }

    /// Return the right node of this node.
    pub(crate) fn right(&self) -> Option<&Box<Node>> {
        self.right.as_ref()
    }

    /// Return whether this node is leaf.
    pub(crate) fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
