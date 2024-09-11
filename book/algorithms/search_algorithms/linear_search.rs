// ANCHOR: linear_search
// ANCHOR: main
fn main() {
    // ANCHOR: input
    let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
    let target = 21;
    // ANCHOR_END: input

    let index = linear_search(array, target);

    match index {
        Some(x) => println!("The element is at index {x}."),
        None => println!("The element is not present in the array."),
    }
}
// ANCHOR_END: main

// ANCHOR: search_function
fn linear_search(array: Vec<i32>, target: i32) -> Option<usize> {

    for (idx, element) in array.into_iter().enumerate() {
        if element == target {
            return Some(idx);
        }
    }

    None
}
// ANCHOR_END: search_function
// ANCHOR_END: linear_search
