// ANCHOR: queue
// ANCHOR: imports
use std::collections::VecDeque;
// ANCHOR_END: imports

fn main() {
    let mut queue = VecDeque::new();

    // ANCHOR: push_back
    // Enqueue operation
    queue.push_back(2);
    queue.push_back(34);
    queue.push_back(8);
    queue.push_back(63);
    queue.push_back(12);
    // ANCHOR_END: push_back

    println!("Queue: {:?}", queue);

    // ANCHOR: pop_front
    // Dequeue operation    
    let popped = queue.pop_front();
    println!("Popped value: {}", popped.unwrap());
    // ANCHOR_END: pop_front

     // ANCHOR: front
     // Peek operation
     let front = queue.front();
     println!("Front value: {}", front.unwrap());
     // ANCHOR_END: front
}
// ANCHOR_END: queue
