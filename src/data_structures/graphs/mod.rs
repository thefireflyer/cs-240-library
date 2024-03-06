///////////////////////////////////////////////////////////////////////////////

use core::fmt;
use std::collections::{HashMap, HashSet};

use std::hash::Hash;

///////////////////////////////////////////////////////////////////////////////

pub mod directed_graph;
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

pub fn depth_first_search<T: Graph>(
    graph: T,
) -> (Vec<<T as Graph>::Item>, Vec<<T as Graph>::Item>, bool)
where
    // T: fmt::Debug,
    // T::Item: fmt::Debug,
    T::Item: Eq + Hash + Clone,
{
    let mut roots = vec![];
    let mut order = vec![];
    let mut cyclic = false;
    let mut perm_mark: HashSet<T::Item> = HashSet::new();
    let mut temp_mark: HashSet<T::Item> = HashSet::new();

    // println!();
    // println!(">> {:?}", graph);

    for origin in graph.get_all() {
        if !perm_mark.contains(&origin) && !temp_mark.contains(&origin) {
            roots.push(origin.clone());
            dfs_visit(
                &graph,
                origin.clone(),
                &mut perm_mark,
                &mut temp_mark,
                &mut cyclic,
                0,
                &mut order,
            );
        }
    }

    order.reverse();

    (roots, order, cyclic)
}

//---------------------------------------------------------------------------//

fn dfs_visit<T: Graph>(
    graph: &T,
    node: T::Item,
    perm_mark: &mut HashSet<T::Item>,
    temp_mark: &mut HashSet<T::Item>,
    cyclic: &mut bool,
    level: usize,
    order: &mut Vec<<T as Graph>::Item>,
) where
    // T: fmt::Debug,
    // T::Item: fmt::Debug,
    T::Item: Eq + Hash + Clone,
{
    // source: https://en.wikipedia.org/wiki/Topological_sorting

    // for i in 0..level {
    //     print!("    ");
    // }
    // println!("> {:?}", node);

    if perm_mark.contains(&node) {
        return;
    }

    if temp_mark.contains(&node) {
        *cyclic = true;
        return;
    }

    temp_mark.insert(node.clone());

    for node in graph.get_adj(&node) {
        dfs_visit(graph, node, perm_mark, temp_mark, cyclic, level + 1, order);
    }

    temp_mark.remove(&node);
    perm_mark.insert(node.clone());
    order.push(node.clone());
}

///////////////////////////////////////////////////////////////////////////////