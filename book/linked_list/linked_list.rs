// ANCHOR: so_far
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

    fn push(&mut self, new_node: Option<Box<Node>>) {
        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = new_node;
    }

    fn print_list(&self) {
        let mut current = Some(self);
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = node.next.as_deref();
        }
        println!("None"); // Indicate the end of the list
    }
}

fn main() {
    let mut head = Box::new(Node { val: 1, next: None });
    let node = Some(Box::new(Node { val: 2, next: None }));

    head.next = node;

    let new_node = Node::new(3);

    head.push(new_node);

    println!("{:?}", head);

    head.push(Node::new(4));

    head.print_list();
}
// ANCHOR_END: so_far
