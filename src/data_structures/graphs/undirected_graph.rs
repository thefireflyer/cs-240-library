///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::Hash,
};

use super::{IDefiniteGraph, IGraph, IGraphEdgeMut, IGraphMut};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    // Map (node -> set of adj nodes)
    adj: HashMap<T, HashSet<T>>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T> UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    //-----------------------------------------------------------------------//

    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
        }
    }

    //-----------------------------------------------------------------------//

    fn inner_insert_edge(&mut self, from: &T, to: &T) {
        if let Some(links) = self.adj.get_mut(from) {
            links.insert(to.clone());
        }
    }

    fn inner_remove_edge(&mut self, from: &T, to: &T) {
        if let Some(links) = self.adj.get_mut(from) {
            links.remove(to);
        }
    }

    //-----------------------------------------------------------------------//

    pub fn get_inner(self) -> HashMap<T, HashSet<T>> {
        self.adj
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T> IGraph for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    type Node = T;

    fn get_adj(&self, node: &Self::Node) -> HashSet<Self::Node> {
        self.adj.get(&node).cloned().unwrap_or_default()
    }

    fn contains(&self, item: &Self::Node) -> bool {
        self.adj.contains_key(item)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> IDefiniteGraph for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    fn get_all(&self) -> Vec<Self::Node> {
        self.adj.keys().cloned().collect()
    }
    fn len(&self) -> usize {
        self.adj.keys().len()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> IGraphMut for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    //-----------------------------------------------------------------------//

    fn insert_node(&mut self, node: Self::Node) {
        self.adj.insert(node.clone(), HashSet::new());
    }

    fn remove_node(&mut self, node: Self::Node) {
        let adj = self.get_adj(&node);

        for neighbor in &adj {
            if let Some(links) = self.adj.get_mut(neighbor) {
                links.remove(&node);
            }
        }

        self.adj.remove(&node);
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl<T> IGraphEdgeMut for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone + Default + fmt::Debug,
{
    //-----------------------------------------------------------------------//

    fn insert_edge(&mut self, left: Self::Node, right: Self::Node) {
        self.inner_insert_edge(&left, &right);
        self.inner_insert_edge(&right, &left);
    }

    fn remove_edge(&mut self, left: Self::Node, right: Self::Node) {
        self.inner_remove_edge(&left, &right);
        self.inner_remove_edge(&right, &left);
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    //-----------------------------------------------------------------------//

    use crate::algorithms::graphs::{bfs::breadth_first_search, dfs::depth_first_search};

    use super::*;

    //-----------------------------------------------------------------------//

    #[test]
    fn construction() {
        for i in 0..500 {
            println!("--- case {} ---", i);
            let mut graph = UndirectedGraph::new();

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
                graph.insert_edge(j, i);
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
            let mut graph = UndirectedGraph::new();

            for j in 1..i {
                graph.insert_node(j);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.insert_edge(j, i - j);
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

            for j in 1..i / 2 {
                let adj = graph.get_adj(&j);

                assert!(!adj.contains(&(i - j)));
                assert_eq!(adj.len(), 0);
            }

            let mut graph = UndirectedGraph::new();

            graph.insert_node(i);

            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j);
                graph.insert_edge(j, i);
                assert_eq!(graph.len(), j + 1);
            }

            if i > 7 {
                for j in 1..i - 3 {
                    graph.insert_edge(j, j + 3);
                }

                for j in 1..i - 7 {
                    graph.insert_edge(j, j + 7);
                }

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    println!("{} -> {:?}", j, adj);

                    assert!(adj.contains(&i));
                    assert!(adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));

                    if j > 7 {
                        assert_eq!(adj.len(), 5);
                    } else if j > 3 {
                        assert_eq!(adj.len(), 4);
                    } else {
                        assert_eq!(adj.len(), 3);
                    }
                }

                for j in 1..i - 3 {
                    graph.remove_edge(j, j + 3);
                }

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    assert!(adj.contains(&i));
                    assert!(!adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));
                    if j > 7 {
                        assert_eq!(adj.len(), 3);
                    } else {
                        assert_eq!(adj.len(), 2);
                    }
                }
                graph.remove_node(i);

                for j in 1..i - 7 {
                    let adj = graph.get_adj(&j);

                    assert!(!adj.contains(&i));
                    assert!(!adj.contains(&(j + 3)));
                    assert!(adj.contains(&(j + 7)));
                    if j > 7 {
                        assert_eq!(adj.len(), 2);
                    } else {
                        assert_eq!(adj.len(), 1);
                    }
                }

                for j in 1..i {
                    graph.remove_node(j);
                    assert_eq!(graph.len(), i - j - 1);
                }
            }
        }
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn bfs_search() {
        for i in vec![0, 1, 2, 3] {
            println!("bfs test with {} layers", i);

            let mut graph = UndirectedGraph::new();

            let mut level = vec![];
            for m in 1..i + 1 {
                let mut new_level = vec![];
                for n in 0..m {
                    graph.insert_node(m * m + n);
                    for node in level.clone() {
                        graph.insert_edge(m * m + n, node);
                    }
                    new_level.push(m * m + n);
                }
                level = new_level;
            }

            let tree = breadth_first_search(graph.clone(), 1);
            println!("{:?}\n{:?}", graph, tree);
        }
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn dfs_search() {
        for i in vec![0, 1, 2, 3, 4, 5] {
            println!("dfs test with {} layers", i);

            let mut graph = UndirectedGraph::new();

            let mut level = vec![];
            for m in 1..i + 1 {
                let mut new_level = vec![];
                for n in 0..m {
                    graph.insert_node(m * m + n);
                    for node in level.clone() {
                        graph.insert_edge(m * m + n, node);
                    }
                    new_level.push(m * m + n);
                }
                level = new_level;
            }

            let (roots, order, cyclic) = depth_first_search(graph.clone());
            println!("Graph: {:?}\nOrder: {:?}\nRoots: {:?}", graph, order, roots);
            assert_eq!(cyclic, i > 1);
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
