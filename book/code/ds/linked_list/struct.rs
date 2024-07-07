// ANCHOR: struct
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
// ANCHOR_END: struct
// ANCHOR: impl
