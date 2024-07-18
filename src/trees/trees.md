A tree is a collection of entities called 'nodes' linked together to simulate a hierarchical structure. 
In a real tree, root is at the bottom; in our computer science tree, root is at the top.

e.g. An employee hierarchy in an organization
<!-- TODO add image of example-->

## Entities of a node:

<!-- TODO add image of tree with labels -->

Node: 

Edges: 

Nodes are usually labelled with a data item. It is called *value*. 
The node at the top of the tree is called as the *root node*. 

Child nodes: Nodes that are linked to an upper node, that is closer to the root node, are called child nodes. 

Parent nodes: Nodes that link to multiple nodes on a lower level are called parent nodes

Sibling nodes: Nodes that are on the same level and share a same parent are called sibling nodes

Leaf nodes: Nodes that do not have any children nodes are called leaf nodes. They are also called terminal nodes.

## Relations

### Nodes and Edges
If a tree has 'n' nodes, it has 'n - 1' edges.

### Depth: 
Depth of some node `N` in a tree can be defined as the number of edges in path from `root` to `N`. 

### Height:
Height of some node `N` is equal to the number of edges in longest path from `N` to leaf node.


## Applications
* Naturally hierarchical data
* For organizing data (for quick search, insertion, deletion).
* Trie
* Network routing
* Decision-making
