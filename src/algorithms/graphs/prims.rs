///////////////////////////////////////////////////////////////////////////////

use std::collections::HashSet;

use crate::data_structures::graphs::{
    weighted_graph::WeightedGraph, IGraph, IGraphEdgeWeightedMut, IGraphMut, IWeightedGraph,
};

///////////////////////////////////////////////////////////////////////////////

/// Returns the minimum spanning tree of the given graph reachable from the
/// provided starting node.
///
/// Inputs:
/// - `graph: &T` The graph to simplify
/// - `origin: T::Node` The node to start building from
///
/// Outputs:
/// - `WeightedGraph<<T>::Node, <T>::Weight>` The minimum spanning tree
/// - `T::Weight` The total weight of the minium spanning tree
///
/// Side-effects: N/A
///
pub fn prims<T: IWeightedGraph>(
    graph: &T,
    origin: T::Node,
) -> (WeightedGraph<<T>::Node, <T>::Weight>, T::Weight) {
    // explained in depth in pseudo-code

    let mut res = WeightedGraph::new();

    res.insert_node(origin.clone());

    let mut frontier = HashSet::new();

    for edge in graph.get_adj_weighted(&origin) {
        frontier.insert((origin.clone(), edge));
    }

    let mut total = T::Weight::from(0);

    while let Some((from, (to, weight))) = frontier.iter().min_by_key(|(_, (_, w))| w).cloned() {
        frontier.remove(&(from.clone(), (to.clone(), weight.clone())));
        if !res.contains(&to) {
            res.insert_node(to.clone());

            res.insert_edge_weighted(from.clone(), to.clone(), weight.clone());
            total = total + weight;

            for edge in graph.get_adj_weighted(&to) {
                frontier.insert((to.clone(), edge));
            }
        }
    }

    (res, total)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::data_structures::graphs::{
        self, weighted_graph::WeightedGraph, IGraphEdgeWeightedMut, IGraphMut,
    };

    use super::prims;

    #[test]
    fn test_prims() {
        let mut graph = WeightedGraph::new();
        // https://www.youtube.com/watch?v=cplfcGZmX7I

        graph.insert_node("A");
        graph.insert_node("B");
        graph.insert_node("C");
        graph.insert_node("D");
        graph.insert_node("E");
        graph.insert_node("F");
        graph.insert_node("G");

        graph.insert_edge_weighted("B", "A", 2);
        graph.insert_edge_weighted("B", "C", 4);
        graph.insert_edge_weighted("B", "E", 3);

        graph.insert_edge_weighted("A", "B", 2);
        graph.insert_edge_weighted("A", "C", 3);
        graph.insert_edge_weighted("A", "D", 3);

        graph.insert_edge_weighted("C", "A", 3);
        graph.insert_edge_weighted("C", "B", 4);
        graph.insert_edge_weighted("C", "E", 1);
        graph.insert_edge_weighted("C", "F", 6);

        graph.insert_edge_weighted("D", "A", 3);
        graph.insert_edge_weighted("D", "F", 7);

        graph.insert_edge_weighted("E", "B", 3);
        graph.insert_edge_weighted("E", "C", 1);
        graph.insert_edge_weighted("E", "F", 8);

        graph.insert_edge_weighted("F", "D", 7);
        graph.insert_edge_weighted("F", "C", 6);
        graph.insert_edge_weighted("F", "E", 8);
        graph.insert_edge_weighted("F", "G", 9);

        graphs::fmt(graph.clone());

        println!("prims");
        let (min_spanning_graph, total) = prims(&graph, "B");

        graphs::fmt(min_spanning_graph.clone());
        println!("{:?}\nTotal weight: {:?}", min_spanning_graph, total);

        assert_eq!(total, 24);
    }
}

///////////////////////////////////////////////////////////////////////////////
