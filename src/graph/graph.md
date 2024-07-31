# Graph

A graph is a non-linear data structure consisting of nodes(vertices) and edges. These edges connect the nodes, representing relationships or connections between them.

## Components

* Vertices(Nodes): These are the individual elements within the graph. They can represent people, places, objects, or any abstract entity.
* Edges: These connect pairs of vertices. They can be directed (one-way) or undirected (two-way). Edges may also have weights or labels to represent additional information.

## Types

* **Directed Graph**: Edges have a direction, indicating a one-way relationship between nodes.
* **Undirected Graph**: Edges have no direction, indicating a two-way relationship between nodes.
* **Weighted Graph**: Edges have weights, representing the cost or distance between nodes.
* **Cyclic Graph**: Contains cycles, or closed paths where you can start and end at the same node.
* **Acyclic Graph**: Does not contain cycles, meaning there are no closed paths.

## Representation

Graphs can be represented in various ways, including:

* **Adjacency Matrix**: A two-dimensional array where each element represents an edge between two vertices. The value at position (i, j) represents the weight of the edge between vertex i and vertex j.
* **Adjacency List**: A collection of linked lists or arrays, where each list represents an edge between two vertices. Each list contains the vertices connected by the edge.