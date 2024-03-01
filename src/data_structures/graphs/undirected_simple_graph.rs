///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug},
    hash::Hash,
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Graph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    // Set (nodes)
    nodes: HashSet<T>,
    // Map (node -> set of adj nodes)
    adj: HashMap<T, HashSet<T>>,
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Graph<T>
where
    T: Ord + fmt::Debug + Hash + Clone,
{
    //-----------------------------------------------------------------------//

    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            adj: HashMap::new(),
        }
    }

    //-----------------------------------------------------------------------//

    pub fn insert_node(&mut self, node: T, adj: Vec<T>) {
        self.nodes.insert(node.clone());

        for neighbor in &adj {
            if let Some(links) = self.adj.get_mut(&neighbor) {
                links.insert(node.clone());
            }
        }

        self.adj
            .insert(node.clone(), HashSet::from_iter(adj.into_iter()));
    }

    pub fn remove_node(&mut self, node: &T) -> bool {
        let adj = self.get_adj(&node);

        for neighbor in &adj {
            if let Some(links) = self.adj.get_mut(neighbor) {
                links.remove(&node);
            }
        }

        self.adj.remove(&node);
        self.nodes.remove(&node)
    }

    //-----------------------------------------------------------------------//

    pub fn insert_edge(&mut self, left: &T, right: &T) {
        self.inner_insert_edge(left, right);
        self.inner_insert_edge(right, left);
    }

    pub fn remove_edge(&mut self, left: &T, right: &T) {
        self.inner_remove_edge(left, right);
        self.inner_remove_edge(right, left);
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

    pub fn get_adj(&self, node: &T) -> HashSet<T> {
        self.adj.get(&node).cloned().unwrap_or_default()
    }

    //-----------------------------------------------------------------------//

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    //-----------------------------------------------------------------------//

    pub fn breadth_first_search(&self, origin: &T) -> Vec<Vec<T>> {
        todo!()
    }

    pub fn breadth_first_search_iter(self, origin: T) -> BreadthFirstSearch<T> {
        BreadthFirstSearch {
            graph: self,
            known: HashSet::from_iter(vec![origin.clone()]),
            frontier: vec![origin],
        }
    }

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct BreadthFirstSearch<T>
where
    T: Ord + Clone + Debug + Hash,
{
    graph: Graph<T>,
    known: HashSet<T>,
    frontier: Vec<T>,
}

//---------------------------------------------------------------------------//

impl<T> Iterator for BreadthFirstSearch<T>
where
    T: Ord + Clone + Debug + Hash,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_frontier = vec![];
        for node in &self.frontier {
            for adj in self.graph.get_adj(node) {
                if !self.known.contains(&adj) {
                    self.known.insert(adj.clone());
                    new_frontier.push(adj);
                }
            }
        }
        self.frontier = new_frontier;

        if self.frontier.len() > 0 {
            Some(self.frontier.clone())
        } else {
            None
        }
    }
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
            let mut graph = Graph::new();

            for j in 1..i {
                graph.insert_node(j, vec![]);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.remove_node(&j);
                assert_eq!(graph.len(), i - j - 1);
            }

            graph.insert_node(i, vec![]);
            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j, vec![i]);
                assert_eq!(graph.len(), j + 1);
            }

            for j in 1..i {
                graph.remove_node(&j);
                assert_eq!(graph.len(), i - j);
            }
            graph.remove_node(&i);
            assert_eq!(graph.len(), 0);
        }
    }

    //-----------------------------------------------------------------------//\

    #[test]
    fn test_edges() {
        for i in 0..500 {
            println!("--- case {} ---", i);
            let mut graph = Graph::new();

            for j in 1..i {
                graph.insert_node(j, vec![]);
                assert_eq!(graph.len(), j);
            }

            for j in 1..i {
                graph.insert_edge(&j, &(i - j));
            }

            for j in 1..i {
                let adj = graph.get_adj(&j);

                assert!(adj.contains(&(i - j)));
                assert_eq!(adj.len(), 1);
            }

            for j in 1..i / 2 {
                graph.remove_node(&j);
                assert_eq!(graph.len(), i - j - 1);
            }

            for j in 1..i / 2 {
                let adj = graph.get_adj(&j);

                assert!(!adj.contains(&(i - j)));
                assert_eq!(adj.len(), 0);
            }

            let mut graph = Graph::new();

            graph.insert_node(i, vec![]);

            assert_eq!(graph.len(), 1);

            for j in 1..i {
                graph.insert_node(j, vec![i]);
                assert_eq!(graph.len(), j + 1);
            }

            if i > 7 {
                for j in 1..i - 3 {
                    graph.insert_edge(&j, &(j + 3));
                }

                for j in 1..i - 7 {
                    graph.insert_edge(&j, &(j + 7));
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
                    graph.remove_edge(&j, &(j + 3));
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
                graph.remove_node(&i);

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
                    graph.remove_node(&j);
                    assert_eq!(graph.len(), i - j - 1);
                }
            }
        }
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn bfs_search() {
        for i in vec![0, 1, 2, 3, 100] {
            let mut graph = Graph::new();

            let mut level = vec![];
            for m in 1..i {
                let mut new_level = vec![];
                for n in 1..m {
                    graph.insert_node(m * m + n, level.clone());
                    new_level.push(m * m + n);
                }
                level = new_level;
            }

            for level in graph.breadth_first_search_iter(1) {
                println!("{:?}", level);
            }
        }
    }

    //-----------------------------------------------------------------------//

    #[test]
    fn dfs_search() {}

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////
