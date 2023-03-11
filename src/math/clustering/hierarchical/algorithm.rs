use crate::math::graph::spanning_tree;
use crate::math::graph::spanning_tree::{MinimumSpanningTree, SpanningTree};

pub struct HierarchicalClustering {
    labels: Vec<u32>,
}

impl HierarchicalClustering {
    fn fit<T>(dataset: &[T]) -> Self {
        if dataset.is_empty() {
            return Self { labels: Vec::new() };
        }

        todo!()
    }
}
