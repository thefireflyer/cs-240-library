///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::Hash,
};

use super::{Graph, GraphMut};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    // Map (node -> set of adj nodes)
    adj: HashMap<T, HashSet<T>>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T> DirectedGraph<T>
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
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Graph for DirectedGraph<T>
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

impl<T> GraphMut for DirectedGraph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    //-----------------------------------------------------------------------//

    fn insert_node(&mut self, node: Self::Item, adj: Vec<Self::Item>) {
        self.adj
            .insert(node.clone(), HashSet::from_iter(adj.into_iter()));
    }

    fn remove_node(&mut self, node: Self::Item) {
        self.adj.remove(&node);
    }

    //-----------------------------------------------------------------------//

    fn insert_edge(&mut self, from: Self::Item, to: Self::Item) {
        if let Some(links) = self.adj.get_mut(&from) {
            links.insert(to.clone());
        }
    }

    fn remove_edge(&mut self, from: Self::Item, to: Self::Item) {
        if let Some(links) = self.adj.get_mut(&from) {
            links.remove(&to);
        }
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
            let mut graph = DirectedGraph::new();

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
            let mut graph = DirectedGraph::new();

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

            // TODO: fix, this currently test non-existent nodes
            for j in 1..i / 2 {
                println!("> {:?}", graph);
                let adj = graph.get_adj(&j);
                println!("{} -- {:?}", j, adj);

                assert!(!adj.contains(&(i - j)));
                assert_eq!(adj.len(), 0);
            }

            let mut graph = DirectedGraph::new();

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

                    assert_eq!(adj.len(), 3);
                }

                for j in 1..i - 3 {
                    graph.remove_edge(j, j + 3);
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

    #[test]
    fn bfs_search() {
        for i in vec![0, 1, 2, 3] {
            println!("bfs test with {} layers", i);

            let mut graph = DirectedGraph::new();

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
            let i = i as i32;
            println!("dfs test with {} layers", i);

            let mut graph = DirectedGraph::new();

            let mut level = vec![];
            for m in 1..i + 1 {
                let mut new_level = vec![];
                for n in 0..m {
                    graph.insert_node(m * m + n, level.clone());
                    new_level.push(m * m + n);
                }
                level = new_level;
            }

            let (roots, order, cyclic) = depth_first_search(graph.clone());
            println!("Graph: {:?}\nOrder: {:?}\nRoots: {:?}", graph, order, roots);
            assert!(!cyclic);

            println!("dfs test with {} layers (part 2)", i);

            let mut graph = DirectedGraph::new();

            fn inner(graph: &mut DirectedGraph<i32>, index: &mut i32, level: i32, max: i32) {
                let local = *index;
                graph.insert_node(local, vec![]);
                *index += 1;
                if level < max {
                    graph.insert_edge(local, *index);
                    inner(graph, index, level + 1, max);
                    graph.insert_edge(local, *index);
                    inner(graph, index, level + 1, max);
                }
            }

            let mut index = 0;
            inner(&mut graph, &mut index, 0, i);

            let (roots, order, cyclic) = depth_first_search(graph.clone());
            println!("Graph: {:?}\nOrder: {:?}\nRoots: {:?}", graph, order, roots);
            assert!(!cyclic);
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
