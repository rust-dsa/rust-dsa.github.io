// ANCHOR: binary_search
fn main() {
    let array = vec![2, 9, 12, 18, 21, 26, 32, 37];
    let token = 21;

    let index = binary_search(array, token);

    match index {
        Some(x) => println!("The element is at index {x}"),
        None => println!("The element is not present in the array"),
    }
}

// ANCHOR: search_function
fn binary_search(array: Vec<i32>, token: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if array[mid] == token {
            return Some(mid);
        } else if token < array[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}
// ANCHOR_END: search_function
// ANCHOR_END: binary_search
