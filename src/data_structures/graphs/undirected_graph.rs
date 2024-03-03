///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::Hash,
};

use super::{Graph, GraphMut};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Graph for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    type Item = T;

    fn get_all(&self) -> Vec<Self::Item> {
        self.adj.keys().cloned().collect()
    }

    fn get_adj(&self, node: &Self::Item) -> HashSet<Self::Item> {
        self.adj.get(&node).cloned().unwrap_or_default()
    }

    fn len(&self) -> usize {
        self.adj.keys().len()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> GraphMut for UndirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    //-----------------------------------------------------------------------//

    fn insert_node(&mut self, node: Self::Item, adj: Vec<Self::Item>) {
        for neighbor in &adj {
            if let Some(links) = self.adj.get_mut(&neighbor) {
                links.insert(node.clone());
            }
        }

        self.adj
            .insert(node.clone(), HashSet::from_iter(adj.into_iter()));
    }

    fn remove_node(&mut self, node: Self::Item) {
        let adj = self.get_adj(&node);

        for neighbor in &adj {
            if let Some(links) = self.adj.get_mut(neighbor) {
                links.remove(&node);
            }
        }

        self.adj.remove(&node);
    }

    //-----------------------------------------------------------------------//

    fn insert_edge(&mut self, left: Self::Item, right: Self::Item) {
        self.inner_insert_edge(&left, &right);
        self.inner_insert_edge(&right, &left);
    }

    fn remove_edge(&mut self, left: Self::Item, right: Self::Item) {
        self.inner_remove_edge(&left, &right);
        self.inner_remove_edge(&right, &left);
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    //-----------------------------------------------------------------------//

    use crate::data_structures::graphs::{breadth_first_search, depth_first_search};

    use super::*;

    //-----------------------------------------------------------------------//

    #[test]
    fn construction() {
        for i in 0..500 {
            println!("--- case {} ---", i);
            let mut graph = UndirectedGraph::new();

            for j in 1..i {
                graph.insert_node(j, vec![]);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.remove_node(j);
                assert_eq!(graph.len(), i - j - 1);
            }

            graph.insert_node(i, vec![]);
            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j, vec![i]);
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
                graph.insert_node(j, vec![]);
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

            graph.insert_node(i, vec![]);

            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j, vec![i]);
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
                    graph.insert_node(m * m + n, level.clone());
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
        for i in vec![0, 1, 2, 3] {
            println!("dfs test with {} layers", i);

            let mut graph = UndirectedGraph::new();

            let mut level = vec![];
            for m in 1..i + 1 {
                let mut new_level = vec![];
                for n in 0..m {
                    graph.insert_node(m * m + n, level.clone());
                    new_level.push(m * m + n);
                }
                level = new_level;
            }

            let forest = depth_first_search(graph.clone());
            println!("{:?}\n{:?}", graph, forest);
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
