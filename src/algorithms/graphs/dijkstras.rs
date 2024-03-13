///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};

use crate::{
    algorithms::graphs::dfs,
    data_structures::{
        binary_heap::BinaryHeap,
        graphs::{IDefiniteGraph, IWeightedGraph},
    },
};

///////////////////////////////////////////////////////////////////////////////

pub fn dag<T: IDefiniteGraph + IWeightedGraph + Clone>(
    graph: T,
    origin: &T::Node,
    target: &T::Node,
) -> Option<Vec<T::Node>> {
    let (_, order, cyclic) = dfs::depth_first_search(graph.clone());

    assert!(!cyclic);

    let mut weights: HashMap<T::Node, T::Weight> = Default::default();
    let mut preds: HashMap<T::Node, T::Node> = Default::default();

    let mut found = false;

    for node in order {
        if &node == origin {
            found = true;
            weights.insert(node.clone(), 0.into());

            for (adj, weight) in graph.get_adj_weighted(&node) {
                weights.insert(adj.clone(), weight);
                preds.insert(adj, node.clone());
            }
        } else if found {
            let cur_weight = weights.get(&node).unwrap().clone();

            for (adj, weight) in graph.get_adj_weighted(&node) {
                match (weights.get_mut(&adj), preds.get_mut(&adj)) {
                    (Some(adj_weight), Some(adj_pred))
                        if *adj_weight > weight.clone() + cur_weight.clone() =>
                    {
                        *adj_weight = weight + cur_weight.clone();
                        *adj_pred = node.clone();
                    }
                    (None, None) => {
                        weights.insert(adj.clone(), weight);
                        preds.insert(adj, node.clone());
                    }
                    _ => {}
                }
            }
        } else if node == *target {
            return None;
        }
    }

    if !found {
        return None;
    }

    let mut res = vec![target.clone()];

    let mut cur = preds.get(target);
    while let Some(curr) = cur {
        res.push(curr.clone());
        cur = preds.get(curr);
    }

    res.reverse();

    Some(res)
}

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
        if let Some((node, weight)) = dist.clone().iter().min_by(|(_, a), (_, b)| a.cmp(b)) {
            dist.remove(node);
            known.insert(node.clone());

            for (adj, adj_weight) in graph.get_adj_weighted(node) {
                match (dist.get_mut(&adj), prev.get_mut(&adj)) {
                    (Some(adj_weight), Some(adj_pred))
                        if *adj_weight > adj_weight.clone() + weight.clone() =>
                    {
                        *adj_weight = adj_weight.clone() + weight.clone();
                        *adj_pred = node.clone();
                    }
                    (None, None) => {
                        dist.insert(adj.clone(), adj_weight);
                        prev.insert(adj, node.clone());
                    }
                    _ => {}
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
            self, weighted_graph::WeightedGraph, IGraphEdgeWeightedMut, IGraphMut,
        },
    };

    use super::dag;

    #[test]
    fn test() {
        for i in 0..20 {
            println!("=== Case {} ===\n", i);

            let mut graph = WeightedGraph::new();

            for n in 0..i {
                for m in 0..n {
                    graph.insert_node(n * n + m);

                    for b in 0..n - 1 {
                        graph.insert_edge_weighted((n - 1) * (n - 1) + b, n * n + m, b);
                    }
                }
            }

            graphs::fmt(graph.clone());

            let path = dag(graph.clone(), &1, &((i - 1) * (i - 1)));

            println!(
                "--- DAG pathfinder\nOrigin: {:?}\nTarget: {:?}\nPath: {:?}\n---\n",
                1,
                ((i - 1) * (i - 1)),
                path
            );

            let path = dijkstras(&graph, &1, &((i - 1) * (i - 1)));

            println!(
                "--- Dijkstras\nOrigin: {:?}\nTarget: {:?}\nPath: {:?}\n---\n",
                1,
                ((i - 1) * (i - 1)),
                path
            );

            println!("===");
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
