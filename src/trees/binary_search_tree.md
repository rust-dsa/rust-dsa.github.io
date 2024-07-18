# Binary search tree

As we already know, searching for a particular element in a collection is a recurring task in computer science. If the data is stored in an unsorted array, then to find the item in question, we'll need to check each element of the array until the correct one is found or the array is exhausted. On average, this will take `O(n/2)` checks, and in the worst case, it will take `O(n)` checks. If the collection is large, such as contact information of every person in a particular area, that will take a long time to search. 

This problem can be optimized by sorting the data and then searching for the item in question using binary search. However, that introduces the overhead of sorting the data, which is not always possible, and maintaining a sorted array if items are inserted or removed. But what if we can speed up the storing and searching process without needing to maintain a sorted array?

**A binary search tree** (BST) is a binary tree where each node has a comparable value; for any given node, all elements in its left subtree are less than the node, and all the elements in its right subtree are greater than the node. This property is known as the BST property.


## Building a binary search tree

## Searching in a binary search tree

## Deleting nodes from a binary search tree

## Time complexity