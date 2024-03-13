///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};

use crate::data_structures::graphs::IWeightedGraph;

///////////////////////////////////////////////////////////////////////////////

pub fn dijkstras<T: IWeightedGraph>(
    graph: &T,
    origin: &T::Node,
    target: &T::Node,
) -> Option<Vec<T::Node>> {
    let mut dist: HashMap<T::Node, T::Weight> = HashMap::new();
    let mut prev: HashMap<T::Node, T::Node> = HashMap::new();

    let mut known: HashSet<T::Node> = HashSet::new();

    dist.insert(origin.clone(), 0.into());

    while !known.contains(target) {
        if let Some((node, weight)) = dist.clone().into_iter().min_by_key(|(_, w)| w.clone()) {
            dist.remove(&node);
            if !known.contains(&node) {
                known.insert(node.clone());

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
