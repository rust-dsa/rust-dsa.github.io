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

### Important Methods

#### `new`
```rust,ignore
impl Node {
    fn new(val: i32) -> Option<Box<Node>> {
        Some(Box::new(Node { val, next: None }))
    }
}
```

We'll create a node using our new method. Since, we cannot directly assign this value to head as it will overwrite the address of the previous node. We will have to access `node` through head and then link the new node to the previous node. We first check if the pattern matches of head node, i.e., if it hold Some value or None. Next we'll borrow the inner value of Some mutably. This will allows us ot modify the contents of `next_node`.

```rust
    let new_node: Option<Box<Node>> = Node::new(3);

    if let Some(ref mut next_node) = head.next {
        next_node.next = new_node;
    }
    println!("{:?}", head);
```

#### `push`

Do you see the pattern for inserting a node in a linked list? Let's articulate it:

1. Start at the head node of the linked list.
2. Traverse the list:
   - Check if the current node's `next` is `None`.
   - If not, move to the next node by reassigning the current node to its `next`.
3. Continue until you find a node where `node.next` is `None` (the last node).
4. Create a new node with the desired value and set the `next` of the last node to point to this new node.

Let's try to write this algorithm by creating the `insert` method

```rust,ignore
    fn push(&mut self, new_node: Option<Box<Node>>) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = new_node;
    }
```

#### `print_list`

We'll create a method that helps us traverse through the list.
Since the method is part of `struct Node` and not `Option<Box<Node>>`, we'll turn it into an Option, so that it is easier to access through `while let` loop. Then using said loop, we'll access `val` and print it, and then assign `next` as current node.

```rust,ignore
fn print_list(&self) {
    let mut current = Some(self);
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_deref();
        }
        println!("None");
    }
```
> The `node.next` is of type `Option<Box<Node>>`, but `current` is expected to be of type `Option<&Node>`. Without `as_deref()`, you can't directly assign `node.next` to `current` because they are different types.


#### `remove_last`

The idea is simple: unlink the last node with the second last node.

```rust,ignore
fn remove_last(&mut self) {
    if self.next.is_none() {
        // If the list is empty or has only one node
        return;
    }

    let mut current = self;
    while let Some(ref mut next) = current.next {
        if next.next.is_none() {
            // We found the second last node
            current.next = None; // Unlink the last node
            return;
        }
        current = next;
    }
}
```

Alright, now that we have written a function that creates, reads, updates and deletes a list node, we'll move on to the next step of design: optimization.

Let's take a look at the code we've written so far:

```rust
{{#include ./linked_list.rs:so_far}}