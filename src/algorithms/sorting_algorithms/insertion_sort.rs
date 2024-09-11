// ANCHOR: insertion_sort
fn main() {
    let array = vec![5, 4, 3, 2, 1];
    let sorted_array = insertion_sort(array);
    println!("{:?}", sorted_array);
}

// ANCHOR: function
fn insertion_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
    array
}
//ANCHOR_END: function
//ANCHOR_END: insertion_sort
