// ANCHOR: so_far
use std::mem;

#[derive(Debug)]
#[allow(dead_code)]

struct List {
    head: Option<Box<Node>>,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl List {
    // ANCHOR: new
    pub fn new() -> Self {
        List { head: None }
    }
    // ANCHOR_END: new

    // ANCHOR: push
    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val: val,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }
    // ANCHOR_END: push

    // ANCHOR: pop
    pub fn pop(&mut self) -> Option<i32> {
        match &self.head {
            None => {
                return None;
            },
            Some(node) {

            }
        }
    }
}

fn main() {
    let mut ll = List::new();
    ll.push(7);
}
// ANCHOR_END: so_far
