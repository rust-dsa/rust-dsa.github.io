# Building a linked list

Now it's time to link Nodes together to create a simple linked list!

### Creating a Head Node

We'll store the address of head node in `Box` pointer, and initialize a node.

```rust,ignore
let head = Box::new(Node {
    val: 1,
    next: None
});
```

### Linking a node
Here, we create a head pointer that points to Node that contains `val = 1` and is not linked to next Node.
The next step is to link one more node to our list. We do this by updating the value of `next` to that of the newly created node.

```rust,ignore
    let mut head = Box::new(Node {
        val: 1,
        next: None
    });
    let node = Some(Box::new(Node {
        val: 2,
        next: None
    }));

    head.next = node;

    println!("{:?}", head);
```

Let's see what we are doing in order to add another node to our linked list.
1. Create a node: `Node` and initialize its value
2. Create another next_node: `Node`, which will hold another value.
3. Make the node mutable so that we can change its `next` value.
4. Assign the address of next_node to the `node.next`.
5. Repeat steps 2 - 4 for every addtion of a node.

This seems is not the optimal way at all. This is not the engineerin way.
So what we'll do is we'll create a new struct which will hold the address of the head node, and also hold the methods that we'll use on the linked list.

## Linked list

To make a linked list more accessible, we'll create another struct called List, which will contain the address of a head node. This head will be of `Option<Box<Node>>` type. We have used the Option enum here so that we could represent an empty list using `None` variant.

```rust,ignore
pub struct List {
    head: Option<Box<Node>>
}
```

The `pub` keyword implies the struct can be acessed by anyone.


Let's see what we have created til now.
1. a Node data structure that holds a value, and the address of next node
2. a List data structure that holds the value of the first node.

