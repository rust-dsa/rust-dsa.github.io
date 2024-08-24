fn main() {
    // ANCHOR: slice_example
    let one_piece: &str = "I am gonna be the King of the Pirates! ⚓🚢";
    let one_pisu: &str = "海賊王に俺はなる！ ⚓🚢";

    println!("{}\n", one_piece);
    println!("{}", one_pisu);
    // ANCHOR_END: slice_example


    // ANCHOR: slice_from_string
    let greeting: &str = "Hello, world!";
    let sub_str = &greeting[0..5];

    println!("{}", greeting);
    println!("{}", sub_str);
    // ANCHOR_END: slice_from_string
}