///////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use std::hash::Hash;

use crate::data_structures::graphs::IGraph;

///////////////////////////////////////////////////////////////////////////////

/// Returns a map of (node -> shortest path from origin)
pub fn breadth_first_search<T: IGraph>(graph: T, origin: T::Node) -> HashMap<T::Node, Vec<T::Node>>
where
    T::Node: Eq + Hash + Clone,
{
    // let's initialize our tracking variables

    // frontier will keep track of the current layer of unexplored but known
    // nodes
    // to begin with, it only contains our origin
    let mut frontier = vec![origin.clone()];

    // known will keep track of all the nodes we've fully explored.
    // it will also map nodes to the shortest path to get to them
    let mut known: HashMap<T::Node, Vec<T::Node>> = HashMap::new();

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
