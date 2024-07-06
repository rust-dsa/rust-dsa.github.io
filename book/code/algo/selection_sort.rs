// ANCHOR: selection_sort
fn main() {
    let array = vec![45, 7, 12, 33, 19, 48, 26, 36];

    let sorted_array = selection_sort(array);

    println!("Sorted array: {:?}", sorted_array);
}

// ANCHOR: function
fn selection_sort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    let arr_len = array.len();
    
    for i in 0..arr_len - 1 {
        let mut current_min = i;
        for current_idx in i + 1..arr_len {
            if array[current_min] > array[current_idx] {
                current_min = current_idx;
            }
        }
        array.swap(current_min, i);
    }
    array
}
// ANCHOR_END: function
// ANCHOR_END: selection_sort