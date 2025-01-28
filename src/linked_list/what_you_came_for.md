# What you came for

In the previous chapter, we saw practical examples of Linked Lists could be implemented using the collections provided by the Rust collections. Using the `use std::collections::LinkedList;` line of code, we were able to access an inbuilt Linked list and use its corresponding methods in order to manipulate data inside the list. But you may have realized that Linked Lists are similar in usage to vectors. To be more precise, singly linked list are akin to `Vec` data structure in Rust, and doubly linked list are akin to `VecDeque`.

So why do we have two ways to represent a structure of data, which could be represented with the help of a single data structure? And which one is a better way to represent data?

Data structures are engineered for human beings as opposed to computers. What I mean is data structures represent data in such a way, that it becomes intelligible to understand the relationship and accessibility between data points. Data structures are solely there for our understanding at a high level. Computers store data one after other. It is literally one big array.
Even if we create a tree in our program, at the low level, the data will still be stored one after the other if the space is present.
The accessiblity of one data point from other data point may change with each data structure, but the overall low level view of data does not change.

Array and linked lists serve different purposes and in a sense overcome each others drawbacks. While we can use arrays in most situations, some cases require the advantages provided by linked lists.

## Creating your own linked list

In the next sections we'll be going low level and creating our own linked lists. Since Rust has features and rules unique to it, our linked list will look different (on the low level) than the linked lists in languages such as C or Python.

Let's dive in!