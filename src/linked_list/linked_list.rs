#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Option<Box<Node>> {
        Some(Box::new(Node { val, next: None }))
    }
}

fn main() {
    let mut head = Box::new(Node { val: 1, next: None });
    let node = Some(Box::new(Node { val: 2, next: None }));

    head.next = node;

    // ANCHOR: new_node
    let new_node = Node::new(3);

    if let Some(ref mut next_node) = head.next {
        next_node.next = new_node;
    }

    println!("{:?}", head);
    // ANCHOR_END: new_node
}
