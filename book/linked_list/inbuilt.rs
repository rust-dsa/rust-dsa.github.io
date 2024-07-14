// ANCHOR: builtin
// ANCHOR: imports
use std::collections::LinkedList;
// ANCHOR_END: imports

fn main() {

    // ANCHOR: linked_list_definition
    let mut list = LinkedList::new();
    // ANCHOR_END: linked_list_definition

    // ANCHOR: push_back
    list.push_back(2);
    list.push_back(34);
    list.push_back(8);
    list.push_back(63);
    // ANCHOR_END: push_back
    println!("Linked list: {:?}", list);

    // ANCHOR: pop_back
    let popped = list.pop_back();
    // ANCHOR_END: pop_back
    println!("Popped value: {}", popped.unwrap());
}
// ANCHOR_END: builtin