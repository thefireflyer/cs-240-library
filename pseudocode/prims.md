# Prim's Algorithm

## Problem

Find a minimum spanning tree given an initial graph and a starting node.

## Example use case

Let's imagine we're designing a new rail network for a region in Maztica.

We want to service as many people as possible, but our organization has pretty limited funds and everyone still wants better internet as well. 

The cheapest network that still reaches everyone is going to be a minimum spanning tree. This is a perfect use case for Prim's algorithm. We give it an initial graph undirected graph, where every city connects to every other city, each with a weight according to how far it is, and an initial random node. We get back an optimal network for our Tabaxi friends to travel on.

## Solution

```
fn prims(Graph graph, Node origin) -> Graph {

    // Let's start by just initializing our new graph
    Graph res = {}

    // We already know the origin is in our graph, so let's insert it
    res.insert(origin)

    // Similar to Dijkstra's algorithm, we need to keep track of new edges
    Set frontier = {}

    // Let's start by finding the adjacent edges of origin
    for each adjacent edge of origin {
        frontier.insert(edge)
    }

    let total = 0


    // Now, similar to Dijkstra's, we'll find the shortest available edge and look over it
    // This will run until we've searched the entire graph
    while let Some((from, to, weight)) = shortest edge in frontier {

        // Remember we've already checked this edge in the future
        frontier.remove((from, to, weight))

        // double check we're not backtracking on a cycle by checking we haven't already added the new node to our result tree
        if to not in res {

            // let's add the new node to the result tree
            res.insert(to)

            // let's connect the new node with the edge we're current traversing 
            res.insert_edge(from, to, weight)
            
            total += weight

            // this new node has a bunch of unexplored edges so let's add them all to our frontier
            for adjacent edge of to {
                frontier.insert(edge)
            }
        }
    }

    (res, total)

}
```