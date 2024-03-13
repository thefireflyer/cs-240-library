///////////////////////////////////////////////////////////////////////////////

use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    hash::Hash,
    ops::Add,
};

use crate::algorithms::graphs::dfs;

///////////////////////////////////////////////////////////////////////////////

pub mod directed_graph;
pub mod undirected_graph;
pub mod weighted_graph;

///////////////////////////////////////////////////////////////////////////////

/// Core graph trait
pub trait IGraph: Clone {
    type Node: Ord + Eq + Hash + Clone + Default + fmt::Debug;

    fn get_adj(&self, node: &Self::Node) -> HashSet<Self::Node>;

    fn contains(&self, item: &Self::Node) -> bool;
}

//---------------------------------------------------------------------------//

/// Extends the core graph trait with edge weights
pub trait IWeightedGraph: IGraph {
    type Weight: Ord
        + Eq
        + Hash
        + Add<Self::Weight, Output = Self::Weight>
        + Clone
        + From<i32>
        + Default
        + fmt::Debug;

    fn get_adj_weighted(&self, node: &Self::Node) -> HashSet<(Self::Node, Self::Weight)>;
}

///////////////////////////////////////////////////////////////////////////////

/// Extends the core graph trait with definite properties
/// (All nodes are known at any given time)
pub trait IDefiniteGraph: IGraph {
    fn get_all(&self) -> Vec<Self::Node>;

    fn len(&self) -> usize;
}

///////////////////////////////////////////////////////////////////////////////

/// Extends the core graph trait with the ability to create and remove nodes
pub trait IGraphMut: IGraph {
    fn insert_node(&mut self, node: Self::Node);
    fn remove_node(&mut self, node: Self::Node);
}

//---------------------------------------------------------------------------//

/// Extends the core graph trait with the ability to create and remove edges
pub trait IGraphEdgeMut: IGraph {
    fn insert_edge(&mut self, from: Self::Node, to: Self::Node);
    fn remove_edge(&mut self, from: Self::Node, to: Self::Node);
}

//---------------------------------------------------------------------------//

/// Extends the core graph trait with the ability to create and remove weighted edges
pub trait IGraphEdgeWeightedMut: IWeightedGraph {
    fn insert_edge_weighted(&mut self, from: Self::Node, to: Self::Node, weight: Self::Weight);
    fn remove_edge_weighted(&mut self, from: Self::Node, to: Self::Node, weight: Self::Weight);
}

///////////////////////////////////////////////////////////////////////////////

pub fn fmt<T: IDefiniteGraph<Node = U>, U: Debug>(graph: T) {
    let chart = dfs::chart_forest(graph);
    for (root, tree) in chart.trees {
        println!(
            "Root: {:?}\nTree: {:?}\nCyclic: {}\n",
            root, tree.topo, tree.cyclic
        );
    }
}

///////////////////////////////////////////////////////////////////////////////
