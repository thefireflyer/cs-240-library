///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::Hash,
    ops::Add,
};

use super::{IDefiniteGraph, IGraph, IGraphEdgeWeightedMut, IGraphMut, IWeightedGraph};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone,
    W: Ord + fmt::Debug + Hash + Clone,
{
    // Map (node -> set of adj nodes)
    adj: HashMap<T, HashSet<(T, W)>>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone,
    W: Ord + fmt::Debug + Hash + Clone,
{
    //-----------------------------------------------------------------------//

    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> IGraph for WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
    W: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    type Node = T;

    fn get_adj(&self, node: &Self::Node) -> HashSet<Self::Node> {
        self.adj
            .get(&node)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|(n, _)| n)
            .collect()
    }

    fn contains(&self, item: &Self::Node) -> bool {
        self.adj.contains_key(item)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> IWeightedGraph for WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
    W: Ord + fmt::Debug + Hash + Clone + Add<W, Output = W> + From<i32> + Default + fmt::Debug,
{
    type Weight = W;

    fn get_adj_weighted(&self, node: &Self::Node) -> HashSet<(Self::Node, Self::Weight)> {
        self.adj.get(&node).cloned().unwrap_or_default()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> IDefiniteGraph for WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
    W: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    fn get_all(&self) -> Vec<Self::Node> {
        self.adj.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.adj.keys().len()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> IGraphMut for WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
    W: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    //-----------------------------------------------------------------------//

    fn insert_node(&mut self, node: Self::Node) {
        self.adj.insert(node.clone(), HashSet::new());
    }

    fn remove_node(&mut self, node: Self::Node) {
        self.adj.remove(&node);
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T, W> IGraphEdgeWeightedMut for WeightedGraph<T, W>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
    W: Ord + fmt::Debug + Hash + Clone + Add<W, Output = W> + From<i32> + Default + fmt::Debug,
{
    //-----------------------------------------------------------------------//
    fn insert_edge_weighted(&mut self, from: Self::Node, to: Self::Node, weight: Self::Weight) {
        if let Some(links) = self.adj.get_mut(&from) {
            links.insert((to.clone(), weight));
        }
    }

    fn remove_edge_weighted(&mut self, from: Self::Node, to: Self::Node, weight: Self::Weight) {
        if let Some(links) = self.adj.get_mut(&from) {
            links.remove(&(to, weight));
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    //-----------------------------------------------------------------------//

    use super::*;

    //-----------------------------------------------------------------------//

    #[test]
    fn construction() {
        for i in 0..500 {
            println!("--- case {} ---", i);
            let mut graph = WeightedGraph::new();

            for j in 1..i {
                graph.insert_node(j);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.remove_node(j);
                assert_eq!(graph.len(), i - j - 1);
            }

            graph.insert_node(i);
            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j);
                graph.insert_edge_weighted(j, i, 1);
                assert_eq!(graph.len(), j + 1);
            }

            for j in 1..i {
                graph.remove_node(j);
                assert_eq!(graph.len(), i - j);
            }
            graph.remove_node(i);
            assert_eq!(graph.len(), 0);
        }
    }

    //-----------------------------------------------------------------------//\

    #[test]
    fn test_edges() {
        for i in 0..500 {
            println!("--- case {} ---", i);
            let mut graph = WeightedGraph::new();

            for j in 1..i {
                graph.insert_node(j);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.insert_edge_weighted(j, i - j, 2);
            }

            for j in 1..i {
                let adj = graph.get_adj(&j);

                assert!(adj.contains(&(i - j)));
                assert_eq!(adj.len(), 1);
            }

            for j in 1..i / 2 {
                graph.remove_node(j);
                assert_eq!(graph.len(), i - j - 1);
            }

            // TODO: fix, this currently test non-existent nodes
            for j in 1..i / 2 {
                println!("> {:?}", graph);
                let adj = graph.get_adj(&j);
                println!("{} -- {:?}", j, adj);

                assert!(!adj.contains(&(i - j)));
                assert_eq!(adj.len(), 0);
            }

            let mut graph = WeightedGraph::new();

            graph.insert_node(i);

            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j);
                graph.insert_edge_weighted(j, i, 1);
                assert_eq!(graph.len(), j + 1);
            }

            if i > 7 {
                for j in 1..i - 3 {
                    graph.insert_edge_weighted(j, j + 3, 1);
                }

                for j in 1..i - 7 {
                    graph.insert_edge_weighted(j, j + 7, 1);
                }

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    println!("{} -> {:?}", j, adj);

                    assert!(adj.contains(&i));
                    assert!(adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));

                    assert_eq!(adj.len(), 3);
                }

                for j in 1..i - 3 {
                    graph.remove_edge_weighted(j, j + 3, 1);
                }

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    assert!(adj.contains(&i));
                    assert!(!adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));
                    assert_eq!(adj.len(), 2);
                }
                graph.remove_node(i);

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    assert!(adj.contains(&i));
                    assert!(!adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));
                    assert_eq!(adj.len(), 2);
                }

                for j in 1..i {
                    graph.remove_node(j);
                    assert_eq!(graph.len(), i - j - 1);
                }
            }
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
