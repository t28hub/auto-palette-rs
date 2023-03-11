use crate::math::graph::edge::{Edge, WeightedEdge};
use crate::math::number::Float;
use std::collections::{BinaryHeap, HashSet};

/// Trait for spanning tree.
pub trait SpanningTree<F: Float, E: Edge> {
    /// Return total weight of this spanning tree.
    fn weight(&self) -> F;

    /// Return all edges of this graph.
    fn edges(&self) -> &[E];
}

/// Minimum spanning tree struct.
#[derive(Debug, Clone)]
pub struct MinimumSpanningTree<F: Float> {
    weight: F,
    edges: Vec<WeightedEdge<F>>,
}

impl<F> MinimumSpanningTree<F>
where
    F: Float,
{
    /// Build a minimum spanning tree.
    pub fn build<V, WF>(vertices: &[V], weight_fn: WF) -> Self
    where
        WF: Fn(usize, usize) -> F,
    {
        if vertices.is_empty() {
            return Self {
                weight: F::zero(),
                edges: Vec::new(),
            };
        }

        let n_vertices = vertices.len();
        let mut edges = Vec::new();
        let mut attached = HashSet::with_capacity(n_vertices);
        let mut candidates = BinaryHeap::new();
        let mut total_weight = F::zero();
        let mut current_index = n_vertices - 1;
        attached.insert(current_index);
        while attached.len() < n_vertices {
            for index in 0..n_vertices {
                if index == current_index || attached.contains(&index) {
                    continue;
                }

                let weight = weight_fn(current_index, index);
                candidates.push(WeightedEdge::new(current_index, index, weight));
            }

            while let Some(edge) = candidates.pop() {
                if !attached.contains(&edge.v()) {
                    current_index = edge.v();
                    total_weight += edge.weight();
                    edges.push(edge);
                    attached.insert(current_index);
                    break;
                }
            }
        }
        Self {
            edges,
            weight: total_weight,
        }
    }
}

impl<F> SpanningTree<F, WeightedEdge<F>> for MinimumSpanningTree<F>
where
    F: Float,
{
    fn weight(&self) -> F {
        self.weight
    }

    fn edges(&self) -> &[WeightedEdge<F>] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_weighted_edge() {
        let vertices = [0, 1, 2, 3];
        let weight_fn = |u: usize, v: usize| -> f64 {
            let incidence_matrix = [
                [f64::MAX, 1.0, 4.0, 3.0],
                [1.0, f64::MAX, 7.0, 2.0],
                [4.0, 9.0, f64::MAX, 5.0],
                [3.0, 2.0, 5.0, f64::MAX],
            ];
            incidence_matrix[u][v]
        };
        let mst = MinimumSpanningTree::build(&vertices, weight_fn);
        assert_eq!(mst.weight(), 7.0);
        assert_eq!(
            mst.edges(),
            &[
                WeightedEdge::new(3, 1, 2.0),
                WeightedEdge::new(1, 0, 1.0),
                WeightedEdge::new(0, 2, 4.0),
            ]
        );
    }
}
