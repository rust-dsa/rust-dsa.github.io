// ANCHOR: builtin
// ANCHOR: imports
use std::collections::LinkedList;
// ANCHOR_END: imports

fn main() {

    // ANCHOR: linked_list_definition
    let mut list = LinkedList::new();
    // ANCHOR_END: linked_list_definition

    list.push_back(8);
    list.push_back(63);
    println!("Linked list: {:?}", list);

    // ANCHOR: push_front
    list.push_front(34);
    list.push_front(2);
    println!("Linked list: {:?}", list);
    // ANCHOR_END: push_front
    

    // ANCHOR: pop_front
    let popped = list.pop_front();
    // ANCHOR_END: pop_front
    println!("Popped value: {}", popped.unwrap());
}
// ANCHOR_END: builtin

