// ANCHOR: hashmap
// ANCHOR: imports
use std::collections::HashMap;
// ANCHOR_END: imports

fn main() {
    // ANCHOR: new
    let mut map = HashMap::new();
    // ANCHOR_END: new

    // ANCHOR: insert
    map.insert("apple", 7);
    map.insert("banana", 5);
    map.insert("cherry", 3);
    // ANCHOR_END: insert

    // ANCHOR: get
    let fruit = map.get("apple");
    println!("The fruit is {}", fruit.unwrap());
    // ANCHOR_END: get

    // ANCHOR: remove
    let apple = map.remove("apple");
    println!("The apple is {}", apple.unwrap());
    // ANCHOR_END: remove

    println!("The map is {:?}", map);
}
// ANCHOR_END: hashmap