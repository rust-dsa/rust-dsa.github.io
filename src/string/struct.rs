fn main() {
    // ANCHOR: string_init
    // let greet = "Hello world!".to_string();
    let greet = String::from("Hello World!");
    println!("{}", greet);
    // ANCHOR_END: string_init

    // ANCHOR: string_new
    let mut empty_string = String::new();
    // ANCHOR_END: string_new

    // ANCHOR: string_push
    let mut greet_mutable = String::new();
    greet_mutable.push_str("Hello ");
    greet_mutable.push_str("World");
    greet_mutable.push('!');
    println!("{}", greet_mutable);
    // ANCHOR_END: string_push
}