# Linked list

A linked list is a linear data structure that stores a collection of 'nodes' together via links i.e. pointers. A node consists of data value and a pointer to the address of the next node within the linked list. Nodes are not stored at contiguous locations, rather they are linked using pointers to different memory locations.

A linked list starts with a head node which points to the starting node of the linked list. Every node consists of data which holds the value associated with the node and a pointer which holds the memory address of the next node in linked list.

The last node is called the tail node in the list which points to null indicating the end of the list.

## Types of linked list

1. Singly linked list
2. Doubly linked list
3. Circular linked list

Let's understand them in a bit more detail

### Singly linked list

A node in the singly linked list consists of only two parts; data and next pointer. The data part stores the actual data of the node and the next part stores the address of its immediate successor.

A singly linked list can be traversed in only one direction.


**Operations** 

1. `create`: 
2. `traverse`:
3. `insert`:
4. `delete`:

### Doubly linked list

A node in doubly linked list consists of three parts; one part is data, other two parts are previous pointer and next pointer. The previous pointer holds the address of the previous node in linked list, the data part holds the actual data, and the next part holds the address of the next node in the linked list

As we have the address of the next as well as the previous node we can traverse in both directions in the linked list.

## Operations

1. `insert_first`:
2. `insert_last`:
3. `insert_after`:
4. `delete_first`:
5. `delete_last`:
6. `delete_after`:
7. `display_forward`: (forward manner)
8. `display backward`: (reverse manner)

### Circular linked list

Circular linked list is an extension of singly and doubly linked list, in that it connects the last node (tail node) with the first node (head node) of the linked list.

This makes sure that they are connected in a loop and we can recursively access every element.