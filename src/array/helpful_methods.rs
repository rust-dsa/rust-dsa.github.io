// ANCHOR: to_vec
fn convert_array_to_vector(array: [i32]) -> Vec<i32> {
    array.to_vec()
}
// ANCHOR_END: to_vec

// ANCHOR: for_loop
fn find_sum_with_for(vector: Vec<i32>) -> i32 {
    let mut sum = 0;
    for element in vector {
        sum += element;
    }

    sum
}
// ANCHOR_END: for_loop

// ANCHOR: iter_sum
fn find_sum_with_sum(vector: Vec<i32>) -> i32 {
    vector.iter().sum()
}
// ANCHOR_END: iter_sum

// ANCHOR: for_each
fn find_sum_with_for_each(vector: Vec<i32>) -> i32 {
    let mut sum = 0;
    vector.for_each(|element| {
        sum += element;
    });

    sum
}
// ANCHOR_END: for_each