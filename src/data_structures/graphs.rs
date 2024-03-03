///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};

use std::hash::Hash;

pub mod undirected_graph;

///////////////////////////////////////////////////////////////////////////////

pub trait Graph {
    type Item;

    fn get_all(&self) -> Vec<Self::Item>;
    fn get_adj(&self, node: &Self::Item) -> HashSet<Self::Item>;

    fn len(&self) -> usize;
}

///////////////////////////////////////////////////////////////////////////////

pub trait GraphMut: Graph {
    fn insert_node(&mut self, node: Self::Item, adj: Vec<Self::Item>);
    fn remove_node(&mut self, node: Self::Item);

    fn insert_edge(&mut self, from: Self::Item, to: Self::Item);
    fn remove_edge(&mut self, from: Self::Item, to: Self::Item);
}

///////////////////////////////////////////////////////////////////////////////

pub fn breadth_first_search<T: Graph>(graph: T, origin: T::Item) -> HashMap<T::Item, Vec<T::Item>>
where
    T::Item: Eq + Hash + Clone,
{
    let mut frontier = vec![origin.clone()];
    let mut known: HashMap<T::Item, Vec<T::Item>> = HashMap::new();

    known.insert(origin, vec![]);

    while frontier.len() > 0 {
        let mut new_frontier = vec![];
        for node in frontier {
            let mut parents = known.get(&node).unwrap().clone();
            parents.push(node.clone());
            for adj in graph.get_adj(&node) {
                if !known.contains_key(&adj) {
                    known.insert(adj.clone(), parents.clone());
                    new_frontier.push(adj);
                }
            }
        }
        frontier = new_frontier;
    }

    known
}

///////////////////////////////////////////////////////////////////////////////

pub fn depth_first_search<T: Graph>(graph: T) -> HashMap<T::Item, Option<T::Item>>
where
    T::Item: Eq + Hash + Clone,
{
    let mut known: HashMap<T::Item, Option<T::Item>> = HashMap::new();

    for origin in graph.get_all() {
        if !known.contains_key(&origin) {
            known.insert(origin.clone(), None);
            dfs_visit(&graph, origin.clone(), &mut known);
        }
    }

    known
}

//---------------------------------------------------------------------------//

fn dfs_visit<T: Graph>(graph: &T, origin: T::Item, known: &mut HashMap<T::Item, Option<T::Item>>)
where
    T::Item: Eq + Hash + Clone,
{
    for node in graph.get_adj(&origin) {
        if !known.contains_key(&node) {
            known.insert(node.clone(), Some(origin.clone()));
            dfs_visit(graph, node, known);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
