### 1. Built-in linked list data type
Rust provides a built-in linked list data type called `LinkedList`. It is a [doubly linked list](#2-doubly-linked-list), but we can use it as a singly linked list as well.

```rust
{{#include ./builtin.rs:builtin}}
```


We need to import the `LinkedList` from the `std::collections` module:

```rust,ignore
{{#include ./builtin.rs:imports}}
```

To to create a linked list we can use the `new` function:
```rust,ignore
{{#include ./builtin.rs:linked_list_definition}}
```

#### Operations
There are two operations in a singly linked list to interact with the data:
1. `push_back`: This function is used to insert a node at the end of the linked list.
```rust,ignore
{{#include ./builtin.rs:push_back}}
```

2. `pop_back`: This function is used to delete a node from the linked list.
```rust,ignore
{{#include ./builtin.rs:pop_back}}
```


### 2. Doubly linked list
Doubly linked lists extend the functionality of singly linked lists by providing two additional methods: `push_front` and `pop_front`.

`push_front` is used to insert a node at the beginning of the linked list.
```rust,ignore
{{#include ./builtin_doubly.rs:push_front}}
```

`pop_front`: This function is used to delete a node from the linked list.
```rust,ignore
{{#include ./builtin_doubly.rs:pop_front}}
```
Let's implement our new functions:

```rust
{{#include ./builtin_doubly.rs:builtin}}
```
When you run this code, you will notice that the when you `push_back` elements, they are stored in the order in which they are pushed. However, when you `push_front` elements, they are stored in the reverse order. This is because the `push_front` function is inserting the new node at the beginning of the linked list.
```terminal
Linked list: [2, 34, 8, 63]
```
Take a look at the result. The last number you added will be the first.
