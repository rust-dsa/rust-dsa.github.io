//ANCHOR: bubble_sort
fn main() {
    let array = vec![45, 7, 12, 33, 19, 48, 26, 36];

    let sorted_array = bubble_sort(array);

    println!("Sorted array: {:?}", sorted_array);
}

// ANCHOR: function
fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    let arr_len = array.len();
    for i in 0..arr_len - 1{
        for j in 0..arr_len - 1 - i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
    array
}
//ANCHOR_END: function
//ANCHOR_END: bubble_sort
