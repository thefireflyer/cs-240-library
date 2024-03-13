///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::algorithms::graphs;
use crate::data_structures::graphs::{IDefiniteGraph, IGraph};

///////////////////////////////////////////////////////////////////////////////

pub struct ForestChart<T: IDefiniteGraph> {
    pub graph: T,
    pub trees: HashMap<T::Node, TreeChart<T>>,
}

//---------------------------------------------------------------------------//

pub struct TreeChart<T: IGraph> {
    pub cyclic: bool,
    pub topo: Vec<T::Node>,
}

///////////////////////////////////////////////////////////////////////////////

pub fn chart_forest<T: IDefiniteGraph>(graph: T) -> ForestChart<T> {
    let mut marks: HashMap<T::Node, bool> = HashMap::new();
    let mut trees = HashMap::new();

    fn chart_tree<T: IGraph>(
        graph: &T,
        node: &T::Node,
        marks: &mut HashMap<T::Node, bool>,
        trees: &mut HashMap<T::Node, TreeChart<T>>,
        cyclic: &mut bool,
        topo: &mut Vec<T::Node>,
    ) {
        match marks.get(node) {
            Some(mark) if *mark => {
                if let Some(other) = trees.remove(node) {
                    *cyclic = *cyclic && other.cyclic;
                    topo.extend(other.topo.into_iter());
                }
                return;
            }
            Some(_) => {
                if let Some(other) = trees.remove(node) {
                    topo.extend(other.topo.into_iter());
                }
                *cyclic = true;
                return;
            }
            None => {}
        }

        marks.insert(node.clone(), false);

        for node in graph.get_adj(&node) {
            chart_tree(graph, &node, marks, trees, cyclic, topo);
        }

        marks.insert(node.clone(), true);

        topo.push(node.clone());
    }

    for root in graph.get_all() {
        if !marks.contains_key(&root) {
            let mut cyclic = false;
            let mut topo = vec![];
            chart_tree(
                &graph,
                &root,
                &mut marks,
                &mut trees,
                &mut cyclic,
                &mut topo,
            );

            trees.insert(root, TreeChart { cyclic, topo });
        }
    }

    ForestChart { graph, trees }
}

///////////////////////////////////////////////////////////////////////////////

/// Returns a tuple containing:
/// - The roots used in searching
/// - A topologically ordered vector of all the nodes in the forest
/// - A boolean on whether the graph is cyclical
///
/// The topological ordering will only be valid for acyclic forests
pub fn depth_first_search<T: IDefiniteGraph>(
    graph: T,
) -> (HashSet<<T as IGraph>::Node>, Vec<<T as IGraph>::Node>, bool)
where
    T::Node: Eq + Hash + Clone,
{
    // we'll initialize all of our tracking variables
    let mut roots = HashSet::new();
    let mut order = vec![];

    // we're assuming the graph acyclic to begin with because an empty graph
    // is acyclic, and will skip everything else
    let mut cyclic = false;

    // these are explained more in dfs_visit
    let mut perm_mark: HashSet<T::Node> = HashSet::new();
    let mut temp_mark: HashSet<T::Node> = HashSet::new();

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
fn dfs_visit<T: IGraph>(
    graph: &T,
    node: T::Node,
    perm_mark: &mut HashSet<T::Node>,
    temp_mark: &mut HashSet<T::Node>,
    cyclic: &mut bool,
    level: usize,
    order: &mut Vec<<T as IGraph>::Node>,
    roots: &mut HashSet<<T as IGraph>::Node>,
) where
    T::Node: Eq + Hash + Clone,
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
