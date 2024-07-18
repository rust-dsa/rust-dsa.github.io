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

## Types of Trees

1. General tree: A general tree is a tree with no constraints. It can have any number of nodes and nodes can have any number of children. A general tree is unordered.

2. Binary Tree: A binary tree is an optimized version of general tree. Every node can have either 0, 1, or 2 children. Hence it is called binary tree.

3. Binary Search Tree: A BST is just like a binary tree except with one difference; the data is ordered. The left child node is always smaller than the parent node, and the right child node is always bigger than parent node.

4. AVL tree: AVL stands for Adelson-Velsky and Landis. AVL tree is an extension of binary search tree in that it is self balancing. It refactors the heights of the left and right subtrees to be as close to each other as possible. The balancing of a tree is measured by something called the balancing factor. 