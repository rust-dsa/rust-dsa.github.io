# Building a linked list

Now its time to link Nodes together to create a simple linked list.

### Creating a head.

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

```rust
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

We can also link another node, but before we do that, let's write a method for `Node` that will help us initialize a node.

```rust,ignore
impl Node {
    fn new(val: i32) -> Option<Box<Node>> {
        Some(Box::new(Node { val, next: None }))
    }
}
```

We'll create a node using our new method. Since, we cannot directly assign this value to head as it will overwrite the address of the previous node. We will have to access `node` through head and then link the new node to the previous node.

```rust
    let new_node: Option<Box<Node>> = Node::new(3);

    if let Some(ref mut next_node) = head.next {
        next_node.next = new_node;
    }
    println!("{:?}", head);
```

We first check if the pattern matches of head node, i.e., if it hold Some value or None. Next we'll borrow the inner value of Some mutably. This will allows us ot modify the contents of `next_node`.