# Dijkstra's Algorithm

## Problem

Given a graph, $G$ of $n$ nodes, find a shortest path, $P$, from node $A$ to node $B$.

The length of a path is the sum of the edge weights between each node in the path.

## Example use case

Let's imagine we're building a self-driving car. We have almost everything working, but it keeps driving really slow routes. One option would be to use Dijkstra's Algorithm. If we have a map of the city, we could create a graph representing each road. Each edge in the graph would have a weight according to how long the stretch of road is. If we provide Dijkstra's with this graph, our car's current position, and our destination, we'll get back an optimal path across the city. Though, under more time constrained conditions it would probably be better to use something like A* or potentially even D* if we have frequently changing environments.

## Solution

One solution is called Dijkstra's Algorithm.

The basic premise is that we explore a given set of edges, pick the smallest, and add its adjacent edges to our overall set of edges. We also track travel times for various nodes.

Depending on the data structures used, this can pretty efficiently find a shortest path in any weighted graph with non-negative edge weights.

We'll need a more concrete algorithm to figure out why it works.

Let's start with our input:
- We need the actual graph itself
- We need a starting point
- And we need an end point

We also need to return a list of nodes as our path.

So we could write something like:

`fn route(Node origin, Node target, Graph graph) -> [Node]`

Now let's figure out what we need to keep track of:
- The nodes we've already explored
- The distances to those nodes
- The way we got to those nodes (we can just store the previous node and backtrack)
- The edges available to us

One key idea in Dijkstra's algorithm is that we can store a specialized heap for tracking both distances to nodes and available edges.

`Heap dist = {}`

We'll use a simple map to keep track of previous nodes:

`Map prev = {}`

And we'll use a set to track the nodes we've already explored:

`Set known = {}`

We start off with an origin point and we already know it takes 0 time to get there so let's insert it into our heap.

`dist.insert((origin, 0))`

Now, we need our main function body. It should continue until we've explored the whole graph or we find the target.
We can check graph exhaustion inside the loop, so let's just use a while loop like this:

`while target not in known:`

Now, the key idea here is that we can efficiently run `pop_min` on our `dist` heap to get the shortest available path. Actually, if `pop_min` returns `None`, that is the heap is empty, that also means we've exhausted our graph. Let's handle this as well:

```
if let Some(node) = dist.pop_min():
    <body>
else:
    return None
```

So we'll just return `None` if we couldn't find a path.

Now let's figure out the inner body.

The first step is removing the node we're now looking at from our unexplored pile:

`dist.remove(node)`

Next, let's check we haven't already explored it:

```
if node not in known:
    <body>
```

Okay, we found a new node, let's add it to our known pile. 

`known.insert(node)`

We have a bunch of new edges available to us now and we track those through the `dist` map. So, let's go and update those. We'll set it up so that nodes we didn't know existed before get the current path weight, and nodes we've already found will only be updated if this current path is better than any previous path. We also will need to remember to update our backtracking map.

```
for each adjacent (adj, weight) of node:
    if dist[adj] is null or dist[adj] > weight + dist[node]:
        dist[adj] = weight + dist[node]
        prev[adj] = node
```

And that's pretty much it!

After everything else, we just backtrack using our map:

```
let res = [target]

let cur = prev[target];
while let Some(curr) = cur {
    res.push(curr)
    cur = prev[curr]
}

res.reverse()

Some(res)
```

So all together:

```
fn route(Node origin, Node target, Graph graph) -> [Node] {

    Heap dist = {}
    Map prev = {}
    Set known = {}

    dist.insert((origin, 0))

    while target not in known:
        if let Some(node) = dist.pop_min():
            if node not in known:
                
                known.insert(node)

                for each adjacent (adj, weight) of node:
                    if dist[adj] is null or dist[adj] > weight + dist[node]:
                        dist[adj] = weight + dist[node]
                        prev[adj] = node

            dist.remove(node)

        else:
            return None
    

    let res = [target]

    let cur = prev[target];
    while let Some(curr) = cur {
        res.push(curr)
        cur = prev[curr]
    }

    res.reverse()

    Some(res)
}
```

