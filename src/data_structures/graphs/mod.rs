///////////////////////////////////////////////////////////////////////////////

use core::fmt;
use std::collections::{HashMap, HashSet};

use std::hash::Hash;

///////////////////////////////////////////////////////////////////////////////

pub mod directed_graph;
pub mod undirected_graph;

///////////////////////////////////////////////////////////////////////////////

/// Core graph trait
pub trait Graph {
    type Item;

    fn get_all(&self) -> Vec<Self::Item>;
    fn get_adj(&self, node: &Self::Item) -> HashSet<Self::Item>;

    fn contains(&self, item: &Self::Item) -> bool;

    fn len(&self) -> usize;
}

///////////////////////////////////////////////////////////////////////////////

/// Extends the core graph trait with ways to modify the graph
pub trait GraphMut: Graph {
    fn insert_node(&mut self, node: Self::Item, adj: Vec<Self::Item>);
    fn remove_node(&mut self, node: Self::Item);

    fn insert_edge(&mut self, from: Self::Item, to: Self::Item);
    fn remove_edge(&mut self, from: Self::Item, to: Self::Item);
}

///////////////////////////////////////////////////////////////////////////////

/// Returns a map of (node -> shortest path from origin)
pub fn breadth_first_search<T: Graph>(graph: T, origin: T::Item) -> HashMap<T::Item, Vec<T::Item>>
where
    T::Item: Eq + Hash + Clone,
{
    // let's initialize our tracking variables

    // frontier will keep track of the current layer of unexplored but known
    // nodes
    // to begin with, it only contains our origin
    let mut frontier = vec![origin.clone()];

    // known will keep track of all the nodes we've fully explored.
    // it will also map nodes to the shortest path to get to them
    let mut known: HashMap<T::Item, Vec<T::Item>> = HashMap::new();

    known.insert(origin, vec![]);

    // while there are nodes that we can still explore...
    while frontier.len() > 0 {
        // we'll create a new list for the next layer of nodes
        let mut new_frontier = vec![];

        // for every node in our frontier...
        for node in frontier {
            // we'll create a variable to keep track of our path called parents
            // we've inheriting the path needed to just get to the current node
            let mut parents = known.get(&node).unwrap().clone();

            // then, to move to new nodes we have to also cross the this node
            parents.push(node.clone());

            // next, we'll iterate through the current node's neighbors
            for adj in graph.get_adj(&node) {
                // if we haven't already explored this node, we'll save it in
                // our new_frontier variable so we can come back later.
                // we'll also save it's path in our known map
                if !known.contains_key(&adj) {
                    known.insert(adj.clone(), parents.clone());
                    new_frontier.push(adj);
                }
            }
        }

        // once we've finished exploring the current frontier, we'll update it
        // to the fresh nodes we've found
        frontier = new_frontier;
    }

    // we'll return our mapping of nodes to paths
    known
}

///////////////////////////////////////////////////////////////////////////////

/// Returns a tuple containing:
/// - The roots used in searching
/// - A topologically ordered vector of all the nodes in the forest
/// - A boolean on whether the graph is cyclical
///
/// The topological ordering will only be valid for acyclic forests
pub fn depth_first_search<T: Graph>(
    graph: T,
) -> (HashSet<<T as Graph>::Item>, Vec<<T as Graph>::Item>, bool)
where
    T::Item: Eq + Hash + Clone,
{
    // we'll initialize all of our tracking variables
    let mut roots = HashSet::new();
    let mut order = vec![];

    // we're assuming the graph acyclic to begin with because an empty graph
    // is acyclic, and will skip everything else
    let mut cyclic = false;

    // these are explained more in dfs_visit
    let mut perm_mark: HashSet<T::Item> = HashSet::new();
    let mut temp_mark: HashSet<T::Item> = HashSet::new();

    // iterate over every node
    for origin in graph.get_all() {
        // if its completely new, let's search it's sub-tree
        if !perm_mark.contains(&origin) && !temp_mark.contains(&origin) {
            // updates roots
            roots.insert(origin.clone());

            // recursively explore the full reachable sub-tree from this node
            dfs_visit(
                &graph,
                origin.clone(),
                &mut perm_mark,
                &mut temp_mark,
                &mut cyclic,
                0,
                &mut order,
                &mut roots,
            );
        }
    }

    // this is explained in dfs_visit, but its faster to build a reverse
    // topological order list and then reverse it then use prepend on a vector
    order.reverse();

    (roots, order, cyclic)
}

//---------------------------------------------------------------------------//

/// Visits all reachable nodes from the provided node.
///
/// Modifies the permanent and temporary marker sets, the cycle conditional,
/// as well as the topological order vector.
fn dfs_visit<T: Graph>(
    graph: &T,
    node: T::Item,
    perm_mark: &mut HashSet<T::Item>,
    temp_mark: &mut HashSet<T::Item>,
    cyclic: &mut bool,
    level: usize,
    order: &mut Vec<<T as Graph>::Item>,
    roots: &mut HashSet<<T as Graph>::Item>,
) where
    T::Item: Eq + Hash + Clone,
{
    // I ended up heavily basing my implementation on Wikipedia's example code:
    // https://en.wikipedia.org/wiki/Topological_sorting

    // check if the current node is a completed sub-tree
    if perm_mark.contains(&node) {
        roots.remove(&node);
        // if so, we can just ignore it
        return;
    }

    // check if the node is in the current sub-tree
    if temp_mark.contains(&node) {
        roots.remove(&node);
        // if so, that means we've found a cycle!
        *cyclic = true;
        // it also means we're already in the process of looking at the node
        // and can ignore it here
        return;
    }

    // we've discovered a new node!
    // we'll mark it as in the current sub-tree or in-processing
    temp_mark.insert(node.clone());

    // we'll iterate over every neighbor node and recursively search each of them
    for node in graph.get_adj(&node) {
        dfs_visit(
            graph,
            node,
            perm_mark,
            temp_mark,
            cyclic,
            level + 1,
            order,
            roots,
        );
    }

    // we've iterated through all the neighbor nodes which means we're done
    // processing this one! Let's update our markers:
    temp_mark.remove(&node);
    perm_mark.insert(node.clone());

    // if this is an acyclic tree and we've just finished processing all of our
    // children, this must be the next item in topological order.
    // that is, if our children depend on us, then we need to go next to make
    // sure all their dependencies are met
    // (this is actually reverse topo order because its faster to use constant
    // time push and then linear time reverse then linear time prepend on each
    // item, for O(n^2) total time)
    order.push(node.clone());
}

///////////////////////////////////////////////////////////////////////////////
