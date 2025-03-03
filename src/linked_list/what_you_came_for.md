# What you came for

In the previous chapter, we explored practical examples of how Linked Lists can be implemented using the collections provided by Rust. By using the `use std::collections::LinkedList;` line of code, we accessed an inbuilt Linked List and utilized its corresponding methods to manipulate the data within the list. You may have noticed that Linked Lists are similar in usage to vectors. To be more precise, singly linked lists are akin to the `Vec` data structure in Rust, while doubly linked lists are similar to `VecDeque`.

So, why do we have two ways to represent a structure of data that could be represented with a single data structure? Which one is the better way to represent data?

Data structures are engineered for human beings rather than computers. What I mean is that data structures represent data in a way that makes it intelligible to understand the relationships and accessibility between data points. Data structures exist solely for our understanding at a high level. Computers store data one after another; it is literally one big array. Even if we create a tree in our program, at the low level, the data will still be stored sequentially if space is available. The accessibility of one data point from another may change with each data structure, but the overall low-level view of data does not change.

Arrays and linked lists serve different purposes and, in a sense, overcome each other's drawbacks. While we can use arrays in most situations, some cases require the advantages provided by linked lists.

## Creating your own linked list

In the next sections, we'll delve into the low-level details and create our own linked lists. Since Rust has features and rules unique to it, our linked list will look different (at the low level) from the linked lists in languages such as C or Python.

Let's dive in!