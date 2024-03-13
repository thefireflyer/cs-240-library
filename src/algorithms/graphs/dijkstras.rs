///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};

use crate::data_structures::graphs::IWeightedGraph;

///////////////////////////////////////////////////////////////////////////////

/// Returns a shortest path from `origin` to `target` in `graph` if it exists
///
/// Inputs:
/// - `graph: &T` The graph to search through
/// - `origin: &T::Node` The node to start from
/// - `target: &T::Node` The node to try and route to
///
/// Output:
/// - If there exists a path from origin to target in graph
///     - `Some(Vec<T::Node>)` A shortest path from origin to target
/// - Else
///     - `None` No valid path found
///
/// Side-effects: N/A
///
pub fn dijkstras<T: IWeightedGraph>(
    graph: &T,
    origin: &T::Node,
    target: &T::Node,
) -> Option<Vec<T::Node>> {
    // explained in depth in pseudo-code

    // Maps: Node -> the shortest known distance from origin
    let mut dist: HashMap<T::Node, T::Weight> = HashMap::new();

    // Maps: Node -> the node immediately before it in the known shortest path
    let mut prev: HashMap<T::Node, T::Node> = HashMap::new();

    // Set of nodes we've already visited
    let mut known: HashSet<T::Node> = HashSet::new();

    // It doesn't take any distance to get from origin to origin
    dist.insert(origin.clone(), 0.into());

    // Loop until we find the target
    while !known.contains(target) {
        // Find the shortest known edge
        // If it doesn't exist, we've exhausted our graph and will immediately
        // return.
        if let Some((node, weight)) = dist.clone().into_iter().min_by_key(|(_, w)| w.clone()) {
            // Remove the smallest edge
            dist.remove(&node);

            // Check if we've already visited this edge's endpoint
            if !known.contains(&node) {
                // Remember new node
                known.insert(node.clone());

                // This is very messy, but is just the relax operation
                // For every adjacent edge
                // - If it's endpoint is completely new or the previous
                //   shortest path is longer than our new one
                //      - Replace the distance with this node's shortest path
                //          length + the weight of this edge.
                for (adj, edge_weight) in graph.get_adj_weighted(&node) {
                    match (dist.get_mut(&adj), prev.get_mut(&adj)) {
                        (Some(node_weight), Some(adj_pred))
                            if *node_weight > weight.clone() + edge_weight.clone() =>
                        {
                            *node_weight = weight.clone() + edge_weight.clone();
                            *adj_pred = node.clone();
                        }
                        (None, None) if adj != *origin => {
                            dist.insert(adj.clone(), weight.clone() + edge_weight);
                            prev.insert(adj, node.clone());
                        }
                        _ => {}
                    }
                }
            }
        } else {
            return None;
        }
    }

    // messy backtracking code

    let mut res = vec![target.clone()];

    let mut cur = prev.get(target);
    while let Some(curr) = cur {
        res.push(curr.clone());
        cur = prev.get(curr);
    }

    res.reverse();

    Some(res)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{
        algorithms::graphs::dijkstras::dijkstras,
        data_structures::graphs::{
            weighted_graph::WeightedGraph, IGraphEdgeWeightedMut, IGraphMut,
        },
    };

    #[test]
    fn test_dijkstras() {
        let mut graph = WeightedGraph::new();
        // https://www.youtube.com/watch?v=EFg3u_E6eHU

        graph.insert_node("A");
        graph.insert_node("B");
        graph.insert_node("C");
        graph.insert_node("D");
        graph.insert_node("E");
        graph.insert_node("F");
        graph.insert_node("G");

        graph.insert_edge_weighted("A", "C", 3);
        graph.insert_edge_weighted("A", "F", 2);

        graph.insert_edge_weighted("C", "A", 3);
        graph.insert_edge_weighted("C", "F", 2);
        graph.insert_edge_weighted("C", "E", 1);
        graph.insert_edge_weighted("C", "D", 4);

        graph.insert_edge_weighted("F", "A", 2);
        graph.insert_edge_weighted("F", "C", 2);
        graph.insert_edge_weighted("F", "E", 3);
        graph.insert_edge_weighted("F", "B", 6);
        graph.insert_edge_weighted("F", "G", 5);

        graph.insert_edge_weighted("E", "C", 1);
        graph.insert_edge_weighted("E", "F", 3);
        graph.insert_edge_weighted("E", "B", 2);

        graph.insert_edge_weighted("D", "C", 4);
        graph.insert_edge_weighted("D", "B", 1);

        graph.insert_edge_weighted("B", "D", 1);
        graph.insert_edge_weighted("B", "E", 2);
        graph.insert_edge_weighted("B", "F", 6);
        graph.insert_edge_weighted("B", "G", 2);

        graph.insert_edge_weighted("G", "F", 5);
        graph.insert_edge_weighted("G", "B", 2);

        let path = dijkstras(&graph, &"A", &"B");
        println!("{:?}", path);

        assert_eq!(path, Some(vec!["A", "C", "E", "B"]));
    }
}

///////////////////////////////////////////////////////////////////////////////
